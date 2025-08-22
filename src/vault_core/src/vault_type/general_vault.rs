use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, storable::Storable};
use std::{borrow::Cow};

use crate::vault_type::{
    flexible_grid::{FlexGridColumns, FlexGridData},
    secure_notes::SecureNoteMap,
    website_logins::WebsiteUserMap,
};

pub type UserId = String; //hash of Principal
pub type VaultId = String; //hash of Vault

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VaultKey {
    pub user_id: UserId,
    pub vault_id: VaultId,
}
// TODO: Change to serde serialization
impl Storable for VaultKey {
    const BOUND: Bound = Bound::Bounded {
        max_size: 512, 
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode VaultKey"))
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(&bytes, VaultKey).expect("Failed to decode VaultKey")
    }

    fn into_bytes(self) -> Vec<u8> {
        Encode!(&self).expect("Failed to encode VaultKey").into()
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct VaultData {
    pub website_logins: WebsiteUserMap,
    pub secure_notes: SecureNoteMap,
    pub flexible_grid: FlexGridData,
    pub flexible_grid_columns: FlexGridColumns,
    pub vault_name: String,
}

impl Storable for VaultData {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(&self).expect("Failed to encode VaultData"))
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(&bytes, VaultData).expect("Failed to decode VaultData")
    }

    fn into_bytes(self) -> Vec<u8> {
        Encode!(&self).expect("Failed to encode VaultData").into()
    }
}