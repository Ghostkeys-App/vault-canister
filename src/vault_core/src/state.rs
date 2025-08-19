use crate::vault_type::general_vault::{VaultData, VaultKey};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
pub struct GeneralState {
    MEMORY_MANAGER: MemoryManager<DefaultMemoryImpl>,
    VAULTS_MAP: RefCell<StableBTreeMap<VaultKey, VaultData, Memory>>,
}

impl GeneralState {
    pub fn init() -> Self {
        let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
        let vaults_map = RefCell::new(StableBTreeMap::init(
            memory_manager.get(MemoryId::new(0)),
        ));
        Self {
            MEMORY_MANAGER: memory_manager,
            VAULTS_MAP: vaults_map,
        }
    }
}
