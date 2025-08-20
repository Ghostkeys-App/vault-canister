use crate::{
    stable::types::VaultsMap,
    vault_type::general_vault::{UserId, VaultData, VaultId, VaultKey},
};

pub fn _add_or_update_vault(vault_id: VaultId, user_id: UserId, data: VaultData, vm: &VaultsMap) {
    vm.borrow_mut().insert(VaultKey { user_id, vault_id }, data);
}

pub fn _get_vault(vault_key: VaultKey, vm: &VaultsMap) -> Option<VaultData> {
    vm.borrow().get(&vault_key)
}

pub fn _get_all_vaults_for_user(user_id: UserId, vm: &VaultsMap) -> Vec<(VaultId, VaultData)> {
    vm.borrow()
        .iter()
        .filter_map(|entry| {
            let VaultKey {
                user_id: uid,
                vault_id: vid,
            } = entry.key();
            let data = entry.value();
            if uid == &user_id {
                Some((vid.clone(), data.clone()))
            } else {
                None
            }
        })
        .collect()
}

pub fn _delete_vault(vault_key: VaultKey, vm: &VaultsMap) {
    vm.borrow_mut().remove(&vault_key);
}

pub fn _clear_all_user_vaults(user_id: UserId, vm: &VaultsMap) {
    let mut map = vm.borrow_mut();
    let keys_to_delete: Vec<_> = map
        .iter()
        .filter_map(|entry| {
            let VaultKey {
                user_id: uid,
                vault_id: vid,
            } = entry.key();
            (uid == &user_id).then(|| VaultKey {
                user_id: uid.clone(),
                vault_id: vid.clone(),
            })
        })
        .collect();

    for key in keys_to_delete {
        map.remove(&key);
    }
}

// TODO: Redefine how we apply changes to the whole config
pub fn _apply_config_changes(changes: Vec<(UserId, VaultId, VaultData)>, vm: &VaultsMap) {
    let mut map = vm.borrow_mut();
    for (user_id, vault_id, vault_data) in changes {
        map.insert(VaultKey { user_id, vault_id }, vault_data);
    }
}

// Do we need a method for multiple vaults?