use candid::{CandidType, Principal, Deserialize};
use ic_cdk::{api::msg_caller, management_canister::{vetkd_derive_key, VetKDCurve, VetKDDeriveKeyArgs, VetKDDeriveKeyResult, VetKDKeyId}};
use ic_vetkeys::{is_valid_transport_public_key_encoding, DerivedPublicKey, MasterPublicKey};


const KEY_NAME: &str = "test_key_1"; // use "key_1" on mainnet. For BETA will use test_key_1
const DOMAIN: &str = "ghostkeys:v1";
const KEY_CURVE: VetKDCurve = VetKDCurve::Bls12_381_G2;

/*
    Utils and Types for key management. I will build separate mod later, since key management is a complex topic
*/

// Get algo-key info
fn key_id() -> VetKDKeyId {
    VetKDKeyId {
        curve: KEY_CURVE,
        name: KEY_NAME.to_string(),
    }
}

// Derive PBK offline
fn offline_dpk(canister_id: Principal, context: &[u8], key_name: &str) -> DerivedPublicKey {
    let kid = VetKDKeyId { curve: VetKDCurve::Bls12_381_G2, name: key_name.to_string() };
    let mpk = MasterPublicKey::for_mainnet_key(&kid).expect("unknown key_id");
    let can_key = mpk.derive_canister_key(canister_id.as_slice());
    can_key.derive_sub_key(context)
}

// Types with Candid
#[derive(CandidType, Deserialize, Clone)]
pub enum Scope {
    PerCanister,
    PerUser { user: Principal },
    PerOrg { org_id: Vec<u8> }, // TODO: TBD on organization scoping
}

#[derive(CandidType, Deserialize, Clone)]
pub struct GhostkeysVetKdArgs {
    pub input: Vec<u8>, // "vault|rotate|purpose"
    pub scope: Scope, 
    pub transport_public_key: Vec<u8>,
}

// Building generic scope
fn build_context(scope: &Scope) -> Vec<u8> {
    let mut ctx = Vec::with_capacity(1 + DOMAIN.len() + 64);
    ctx.push(DOMAIN.len() as u8);
    ctx.extend_from_slice(DOMAIN.as_bytes());
    match scope {
        Scope::PerCanister => {} // should be empty
        Scope::PerUser { user } => ctx.extend_from_slice(user.as_slice()),
        Scope::PerOrg { org_id } => ctx.extend_from_slice(org_id),
    }
    ctx
}

pub fn storage_user_of(scope: &Scope) -> Principal {
    match scope {
        Scope::PerUser { user } => *user,
        _ => msg_caller(), // PerCanister / PerOrg → attribute to caller
    }
}

pub async fn derive_vetkey(args: GhostkeysVetKdArgs) -> Result<Vec<u8>, String> {
    if !is_valid_transport_public_key_encoding(&args.transport_public_key) {
        return Err("invalid transport_public_key encoding".into());
    }
    if args.input.len() > 1024 {
        return Err("input too large".into());
    }

    let req = VetKDDeriveKeyArgs {
        input: args.input.clone(),
        context: build_context(&args.scope),
        transport_public_key: args.transport_public_key.clone(),
        key_id: key_id(),
    };

    let VetKDDeriveKeyResult { encrypted_key } = vetkd_derive_key(&req)
        .await
        .map_err(|e| format!("vetkd_derive_key failed: {:?}", e))?;

    Ok(encrypted_key) // opaque blob (client will decrypt+verify) | Important: Structure should remain unchanged through rotation
}
