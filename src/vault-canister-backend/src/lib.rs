use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::{
    api::{canister_liquid_cycle_balance, msg_caller}, management_canister::{
        raw_rand, VetKDCurve, VetKDDeriveKeyArgs, VetKDKeyId,
        VetKDPublicKeyArgs,
    }, query, update,
    call::Call
};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableBTreeMap, Storable,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{borrow::Cow, cell::RefCell};
use getrandom::{register_custom_getrandom};

register_custom_getrandom!(getrandom_entropy);
pub fn getrandom_entropy(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    futures::executor::block_on(raw_rand())
        .map_err(|_| getrandom::Error::UNSUPPORTED)
        .map(|_| ())
}

/*
    I'm not sure if this works. TODO: TEST WITH CLIENT. Check if this session thingy works on auth proof.
*/

const DS_AUTH: &[u8] = b"ghostkeys:auth:v1";
const NONCE_TTL_NS: u64 = 120 * 1_000_000_000; // 2 minutes
const KEY_NAME: &str = "test_key_1"; // use "key_1" on mainnet. WE NEED TO SWAP THIS FOR PROD (MAYBE, HAVEN't decided yet for the structure)
const KEY_CURVE: VetKDCurve = VetKDCurve::Bls12_381_G2;

// Threshold at which the canister should request a top-up.
// This must be a value sufficient to serve a request and continue storing data until the top-up is received.
const MIN_CYCLES_BALANCE : u128 = 1_000_000_000;

type Memory = VirtualMemory<DefaultMemoryImpl>;
thread_local! {
    static MM: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static CHALLENGES: RefCell<StableBTreeMap<[u8;32], Challenge, Memory>> = RefCell::new(
        StableBTreeMap::init(MM.with(|m| m.borrow().get(MemoryId::new(0))))
    );

    static SESSIONS: RefCell<StableBTreeMap<[u8;32], Session, Memory>> = RefCell::new(
        StableBTreeMap::init(MM.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    static CANISTER_OWNERS: RefCell<CanisterOwners> = RefCell::new(CanisterOwners {
        user: Principal::anonymous(),
        controller: Principal::anonymous(),
    });
}

struct CanisterOwners {
    user: Principal,
    controller: Principal,
}

#[ic_cdk_macros::init]
fn canister_init(arg: Vec<u8>) {
    let (user, controller): (Principal, Principal) = Decode!(&arg, (Principal, Principal))
        .expect("Failed to decode canister init arguments");
    CANISTER_OWNERS.with(|m| {
        m.borrow_mut().user = user;
        m.borrow_mut().controller = controller;
    });
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Challenge {
    owner: Principal,
    vault: Principal,
    issued_ns: u64,
    expires_ns: u64,
    used: bool,
}
impl Storable for Challenge {
    const BOUND: Bound = Bound::Bounded {
        max_size: (200),
        is_fixed_size: (false),
    };
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode Challenge"))
    }
    fn into_bytes(self) -> Vec<u8> {
        Encode!(&self).expect("Failed to encode Challenge").into()
    }
    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(&bytes, Challenge).expect("Failed to decode Challenge")
    }
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Session {
    owner: Principal,
    vault: Principal,
    expires_ns: u64,
}
impl Storable for Session {
    const BOUND: Bound = Bound::Bounded {
        max_size: (200),
        is_fixed_size: (false),
    };
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode Storable"))
    }
    fn into_bytes(self) -> Vec<u8> {
        Encode!(&self).expect("Failed to encode Storable").into()
    }
    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(&bytes, Session).expect("Failed to decode Storable")
    }
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct IssuedChallenge {
    pub nonce: Vec<u8>, // 16 bytes
    pub expires_ns: u64,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct DerivedPubkey {
    pub public_key: Vec<u8>, // 96 bytes (BLS12-381 G2) - I hope I'm correct here
}

fn key_id() -> VetKDKeyId {
    VetKDKeyId {
        curve: KEY_CURVE,
        name: KEY_NAME.to_string(),
    }
}

fn make_context(owner: Principal, vault: Principal) -> Vec<u8> {
    [DS_AUTH.len() as u8]
        .into_iter()
        .chain(DS_AUTH.iter().cloned())
        .chain(owner.as_slice().iter().cloned())
        .chain(vault.as_slice().iter().cloned())
        .collect()
}

fn challenge_key(owner: Principal, vault: Principal, nonce: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(owner.as_slice());
    h.update(vault.as_slice());
    h.update(nonce);
    h.update(DS_AUTH);
    h.finalize().into()
}

/// Maintains the status of the canister, performing any tasks needed to keep it operational.
/// Every update call should trigger this function.
/// These include:
/// - [ ] Cleaning up expired challenges
/// - [x] Checking the cycles balance and requesting a topup if needed
fn maintain_status() {
    let can_cycles = canister_liquid_cycle_balance();

    if can_cycles < MIN_CYCLES_BALANCE {
        // get the owning principal (currently factory can)
        let owner = CANISTER_OWNERS.with_borrow(|m| m.controller);
        // call the top_up method on the canister
        let _result = Call::unbounded_wait(
            owner,
            "top_up",
        );
    }
}

#[update]
pub async fn issue_auth_challenge(vault: Principal) -> IssuedChallenge {
    maintain_status();

    let rand_bytes: Vec<u8> = raw_rand().await.unwrap_or_else(|_| {
        // fallback entropy (Vec<u8>)
        let t = ic_cdk::api::time().to_le_bytes();
        (0..32).map(|i| t[i % t.len()]).collect()
    });

    // 16-byte nonce from the 32 bytes
    let nonce: Vec<u8> = rand_bytes.into_iter().take(16).collect();

    let now = ic_cdk::api::time();
    let expires = now + NONCE_TTL_NS;
    let ch = Challenge {
        owner: msg_caller(),
        vault,
        issued_ns: now,
        expires_ns: expires,
        used: false,
    };

    CHALLENGES.with(|m| {
        m.borrow_mut()
            .insert(challenge_key(ch.owner, vault, &nonce), ch);
    });

    IssuedChallenge {
        nonce,
        expires_ns: expires,
    }
}

#[update]
pub async fn vetkd_public_key(vault: Principal) -> Result<DerivedPubkey, String> {
    maintain_status();

    let req = VetKDPublicKeyArgs {
        canister_id: None,
        context: make_context(msg_caller(), vault),
        key_id: key_id(),
    };

    // typed helper from ic_cdk::management_canister
    let reply = ic_cdk::management_canister::vetkd_public_key(&req)
        .await
        .map_err(|e| format!("vetkd_public_key failed: {e:?}"))?;

    Ok(DerivedPubkey {
        public_key: reply.public_key,
    })
}

#[update]
pub async fn vetkd_encrypted_key(
    vault: Principal,
    nonce: Vec<u8>,
    transport_public_key: Vec<u8>,
) -> Result<Vec<u8>, String> {
    maintain_status();

    let owner = msg_caller();
    let key = challenge_key(owner, vault, &nonce);
    let ch = CHALLENGES
        .with(|m| m.borrow().get(&key))
        .ok_or("challenge not found")?;
    let now = ic_cdk::api::time();
    if ch.owner != owner || ch.vault != vault {
        return Err("challenge mismatch".into());
    }
    if ch.used {
        return Err("challenge already used".into());
    }
    if now > ch.expires_ns {
        return Err("challenge expired".into());
    }

    let req = VetKDDeriveKeyArgs {
        input: nonce.clone(),
        context: make_context(owner, vault),
        transport_public_key,
        key_id: key_id(),
    };

    let reply = ic_cdk::management_canister::vetkd_derive_key(&req)
        .await
        .map_err(|e| format!("vetkd_derive_key failed: {e:?}"))?;

    Ok(reply.encrypted_key)
}

#[update]
pub async fn verify_auth_proof(
    vault: Principal,
    nonce: Vec<u8>,
    proof_sig: Vec<u8>,
) -> Result<Vec<u8>, String> {
    maintain_status();

    let owner = msg_caller();
    let k = challenge_key(owner, vault, &nonce);
    let mut challenge = CHALLENGES
        .with(|m| m.borrow().get(&k))
        .ok_or("challenge not found")?;
    let now = ic_cdk::api::time();
    if challenge.owner != owner || challenge.vault != vault {
        return Err("challenge mismatch".into());
    }
    if challenge.used {
        return Err("challenge already used".into());
    }
    if now > challenge.expires_ns {
        return Err("challenge expired".into());
    }

    let pk_reply = ic_cdk::management_canister::vetkd_public_key(&VetKDPublicKeyArgs {
        canister_id: None,
        context: make_context(owner, vault),
        key_id: key_id(),
    })
    .await
    .map_err(|e| format!("vetkd_public_key failed: {e:?}"))?;

    let derived_pk = ic_vetkeys::DerivedPublicKey::deserialize(&pk_reply.public_key)
        .map_err(|e| format!("bad public key: {e:?}"))?;

    let ok = ic_vetkeys::verify_bls_signature(&derived_pk, &nonce, &proof_sig);
    if !ok {
        return Err("invalid proof".into());
    }

    challenge.used = true;
    CHALLENGES.with(|m| {
        m.borrow_mut().insert(k, challenge);
    });

    let rnd: Vec<u8> = raw_rand()
        .await
        .map_err(|e| format!("raw_rand failed: {e:?}"))?;

    let mut sid = [0u8; 32];
    let n = std::cmp::min(sid.len(), rnd.len());
    sid[..n].copy_from_slice(&rnd[..n]);

    let session = Session {
        owner,
        vault,
        expires_ns: now + 30 * 60 * 1_000_000_000,
    };
    SESSIONS.with(|m| {
        m.borrow_mut().insert(sid, session);
    });

    Ok(sid.to_vec())
}

#[query]
pub fn session_info(session_id: Vec<u8>) -> Option<Session> {
    if session_id.len() != 32 {
        return None;
    }
    let mut sid = [0u8; 32];
    sid.copy_from_slice(&session_id);
    SESSIONS
        .with(|m| m.borrow().get(&sid))
        .filter(|s| ic_cdk::api::time() <= s.expires_ns)
}

ic_cdk::export_candid!();
