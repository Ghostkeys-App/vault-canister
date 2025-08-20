use ic_cdk_macros::{query, update};
use vault_core::{
    api::*,
    stable::types::GeneralState,
    vault_type::general_vault::{UserId, VaultData, VaultId, VaultKey},
};

thread_local! {
    static GENERAL_STATE: GeneralState = GeneralState::init();
}

#[query]
fn get_vault(user_id: UserId, vault_id: VaultId) -> Option<VaultData> {
    GENERAL_STATE.with(|state| _get_vault(VaultKey { user_id, vault_id }, &state.vaults_map))
}

#[query]
fn get_all_vaults_for_user(user_id: UserId) -> Vec<(VaultId, VaultData)> {
    GENERAL_STATE.with(|state| _get_all_vaults_for_user(user_id, &state.vaults_map))
}

#[update]
fn add_or_update_vault(user_id: UserId, vault_id: VaultId, vault: VaultData) {
    GENERAL_STATE.with(|state| {
        _add_or_update_vault(vault_id, user_id, vault, &state.vaults_map);
    });
}

#[update]
fn delete_vault(user_id: UserId, vault_id: VaultId) {
    GENERAL_STATE.with(|state| {
        _delete_vault(VaultKey { user_id, vault_id }, &state.vaults_map);
    });
}

#[update]
fn clear_all_user_vaults(user_id: UserId) {
    GENERAL_STATE.with(|state| {
        _clear_all_user_vaults(user_id, &state.vaults_map);
    });
}

#[update]
fn apply_config_changes(changes: Vec<(UserId, VaultId, VaultData)>) {
    GENERAL_STATE.with(|state| {
        _apply_config_changes(changes, &state.vaults_map);
    });
}


ic_cdk::export_candid!();
