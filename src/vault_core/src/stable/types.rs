use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap
};

use crate::vault_type::{
    logins::LoginSiteKey, secure_notes::{SecureNote, SecureNoteKey}, spreadsheet::{SpreadsheetKey, SpreadsheetValue}
};

// Stable memory for vaults
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Stable memory for spreadsheets.
pub type SpreadsheetMap = RefCell<StableBTreeMap<SpreadsheetKey, SpreadsheetValue, Memory>>;

// Stable memory for login data and metadata.
pub type LoginsMap = RefCell<StableBTreeMap<SpreadsheetKey, SpreadsheetValue, Memory>>;
pub type LoginsColumns = RefCell<StableBTreeMap<LoginSiteKey, Vec<u8>, Memory>>;
pub type NotesMap = RefCell<StableBTreeMap<SecureNoteKey, SecureNote, Memory>>;

// Stable memory for canister management 
pub struct CanisterOwners {
    pub controller: Principal,
    pub user: Vec<Principal>,
}
pub type CanisterOwnersState = RefCell<CanisterOwners>;

// Stable memory for KeyManagement. Implementation to hold per-user. Beta will have per-canister. See key_api.rs for specifications.
pub type KeyManagementState = RefCell<StableBTreeMap<String, Vec<u8>, Memory>>;

/*
   General state of the canister, including vaults and other relevant data.
*/
pub struct GeneralState {
    pub memory_manager: MemoryManager<DefaultMemoryImpl>,
    pub canister_owners: CanisterOwnersState,
    pub key_management: KeyManagementState,
    pub spreadsheet_map: SpreadsheetMap,
    pub logins_map: LoginsMap,
    pub logins_columns: LoginsColumns,
    pub notes_map: NotesMap,
}
