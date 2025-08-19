use crate::vault_type::general_vault::{VaultData, VaultKey};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type VaultsMap = RefCell<StableBTreeMap<VaultKey, VaultData, Memory>>;
pub struct GeneralState {
    pub memory_manager: MemoryManager<DefaultMemoryImpl>,
    pub vaults_map: VaultsMap,
}

impl GeneralState {
    pub fn init() -> Self {
        let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
        let vaults_map = RefCell::new(StableBTreeMap::init(
            memory_manager.get(MemoryId::new(0)),
        ));
        Self {
            memory_manager,
            vaults_map,
        }
    }
}
