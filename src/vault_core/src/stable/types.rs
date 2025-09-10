use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap
};

use crate::vault_type::{
    general_vault::{UserId, VaultData, VaultKey}, 
    spreadsheet::{SpreadsheetKey, SpreadsheetValue}, 
    logins::LoginSiteKey
};

// Stable memory for vaults
type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type VaultsMap = RefCell<StableBTreeMap<VaultKey, VaultData, Memory>>;

// Stable memory for spreadsheets.
pub type SpreadsheetMap = RefCell<StableBTreeMap<SpreadsheetKey, SpreadsheetValue, Memory>>;

// Stable memory for login data and metadata.
pub type LoginsMap = RefCell<StableBTreeMap<SpreadsheetKey, SpreadsheetValue, Memory>>;
pub type LoginsColumns = RefCell<StableBTreeMap<LoginSiteKey, Vec<u8>, Memory>>;

// Stable memory for canister management 
pub struct CanisterOwners {
    pub controller: Principal,
    pub user: Vec<Principal>,
}
pub type CanisterOwnersState = RefCell<CanisterOwners>;

// Stable memory for KeyManagement. Implementation to hold per-user. Beta will have per-canister. See key_api.rs for specifications.
pub type KeyManagementState = RefCell<StableBTreeMap<UserId, Vec<u8>, Memory>>;

/*
   General state of the canister, including vaults and other relevant data.
*/
pub struct GeneralState {
    pub memory_manager: MemoryManager<DefaultMemoryImpl>,
    pub vaults_map: VaultsMap,
    pub canister_owners: CanisterOwnersState,
    pub key_management: KeyManagementState,
    pub spreadsheet_map: SpreadsheetMap,
    pub logins_map: LoginsMap,
    pub logins_columns: LoginsColumns,
}
