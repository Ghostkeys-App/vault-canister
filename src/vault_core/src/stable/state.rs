use crate::stable::types::{CanisterOwners, GeneralState};
use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager}, DefaultMemoryImpl, StableBTreeMap
};
use std::{cell::RefCell};

impl GeneralState {
    pub fn init() -> Self {
        let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
        let canister_owners = RefCell::new(CanisterOwners {
            controller: Principal::anonymous(),
            user: Vec::default(),
        });
        let key_management = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(1))));
        let spreadsheet_columns = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(2))));
        let spreadsheet_map = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(3))));
        let logins_map = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(4))));
        let logins_columns = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(5))));
        let notes_map = RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(6))));
        let vault_names_map =  RefCell::new(StableBTreeMap::init(memory_manager.get(MemoryId::new(7))));
        Self {
            memory_manager,
            canister_owners,
            key_management,
            spreadsheet_columns,
            spreadsheet_map,
            logins_map,
            logins_columns,
            notes_map,
            vault_names_map
        }
    }
}
