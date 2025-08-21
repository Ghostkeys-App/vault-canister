use crate::stable::types::{CanisterOwners, GeneralState};
use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::{cell::RefCell};

impl GeneralState {
    pub fn init() -> Self {
        let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
        let vaults_map = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(0))));
        let canister_owners = RefCell::new(CanisterOwners {
            controller: Principal::anonymous(),
            user: Vec::default()
        });
        let key_management = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(1))));
        Self {
            memory_manager,
            vaults_map,
            canister_owners,
            key_management,
        }
    }
}
