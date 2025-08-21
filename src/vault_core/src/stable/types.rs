use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};

use crate::vault_type::general_vault::{VaultData, VaultKey};

// Stable memory for vaults
type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type VaultsMap = RefCell<StableBTreeMap<VaultKey, VaultData, Memory>>;

// Stable memory for canister management 
pub struct CanisterOwners {
    pub controller: Principal,
    pub user: Vec<Principal>,
}
pub type CanisterOwnersState = RefCell<CanisterOwners>;

/*
   General state of the canister, including vaults and other relevant data.
*/
pub struct GeneralState {
    pub memory_manager: MemoryManager<DefaultMemoryImpl>,
    pub vaults_map: VaultsMap,
    pub canister_owners: CanisterOwnersState,
}