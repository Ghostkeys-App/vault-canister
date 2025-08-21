use candid::Principal;
use ic_cdk_macros::{query, update};
use vault_core::{
    api::{
        key_api::{derive_vetkey, storage_user_of, GhostkeysVetKdArgs},
        vault_api::*,
    },
    stable::{
        util::_init_controllers,
        {types::GeneralState, util::maintain_status},
    },
    vault_type::general_vault::{UserId, VaultData, VaultId, VaultKey},
};

thread_local! {
    static GENERAL_STATE: GeneralState = GeneralState::init();
}

// Helper for cost-related. TODO: move
fn maintain_canister_status() {
    GENERAL_STATE.with(|m| {
        maintain_status(&m.canister_owners);
    });
}

#[update]
fn shared_canister_init(user: Principal, controller: Principal) {
    GENERAL_STATE.with(|m| {
        _init_controllers(user, controller, &m.canister_owners);
    });
}

/*
    Key-management Specific Endpoints
*/

#[update]
async fn derive_vetkd_encrypted_key(args: GhostkeysVetKdArgs) -> Result<Vec<u8>, String> {
    maintain_canister_status();
    let scope_copy = args.scope.clone();
    let encrypted_key = derive_vetkey(args).await?;
    let owner_principal = storage_user_of(&scope_copy);

    GENERAL_STATE.with(|st| {
        st.key_management
            .borrow_mut()
            .insert(owner_principal.to_text(), encrypted_key.clone());
    });

    Ok(encrypted_key)
}

/*
    Vault Specific Endpoints
*/

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

#[update]
fn add_user(user: Principal) {
    GENERAL_STATE.with(|state| {
        state.canister_owners.borrow_mut().user.push(user);
    });
}

ic_cdk::export_candid!();
