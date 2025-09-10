use candid::Principal;
use ic_cdk::{api::msg_caller, call::Call, inspect_message};
use ic_cdk_macros::{query, update};

// import tests
#[cfg(test)]
mod test;

use vault_core::{
    api::{
        key_api::{derive_vetkey, retrieve_vetkey_per_user, storage_user_of, GhostkeysVetKdArgs}, serial_api::{_global_sync, _login_data_deletes, _login_data_sync, _login_full_sync, _login_metadata_delete, _login_metadata_sync, _vault_spreadsheet_delete, _vault_spreadsheet_sync}, vault_api::*
    },
    stable::{
        types::GeneralState,
        util::{_init_controllers, _inspect_message, maintain_status},
    },
    vault_type::general_vault::{UserId, VaultData, VaultId, VaultKey},
};

thread_local! {
    static GENERAL_STATE: GeneralState = GeneralState::init();
}

const MAX_VAULTS_PER_USER: u64 = 3;
const MAX_VAULT_SIZE_BYTES: u64 = 1 * 1024 * 1024 * 1024; // 1 GB
const STORAGE_PER_USER: u64 = MAX_VAULTS_PER_USER * MAX_VAULT_SIZE_BYTES;

const MAX_USER_STORAGE: u64 = 400 * 1024 * 1024 * 1024; // 400 GB
const MAX_USERS: u64 = MAX_USER_STORAGE / STORAGE_PER_USER;

// Helper for cost-related. TODO: move
fn maintain_canister_status() {
    GENERAL_STATE.with(|m| {
        maintain_status(&m.canister_owners); // check if we need more cycles
    });
}

#[inspect_message]
fn inspect_message() {
    let always_accept: Vec<String> = vec![
        "shared_canister_init".to_string(), // TODO - needs to be reworked so only the factory can call this, and only once
        "derive_vetkd_encrypted_key".to_string(), // TODO - requires proof of work from caller to prevent canister flooding
        "get_vetkey_for_user".to_string()
    ];
    // call common inspect
    GENERAL_STATE.with(|m| _inspect_message(&always_accept, &m.canister_owners))
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
    let scope_copy = args.scope.clone();
    let owner_principal = storage_user_of(&scope_copy);

    // check we haven't exceeded max users
    GENERAL_STATE.with(|state| {
        let current_users: u64 = state.vaults_map.borrow().len();
        let mut canister_owners = state.canister_owners.borrow_mut();
        if !canister_owners.user.contains(&owner_principal) {
            if current_users == MAX_USERS - 1 {
                // notify the factory canister that we are at capacity, but handle this new user.
                let _ = Call::unbounded_wait(
                    state.canister_owners.borrow().controller,
                    "notify_canister_at_capacity",
                );
            } else if current_users >= MAX_USERS {
                ic_cdk::trap("Canister at max user capacity");
            } else {
                canister_owners
                    .user
                    .push(owner_principal);
            }
        }
    });

    if let Some(existing_key) =
        GENERAL_STATE.with(|st| st.key_management.borrow().get(&owner_principal.to_text()))
    {
        return Ok(existing_key);
    }

    maintain_canister_status();
    let encrypted_key = derive_vetkey(args).await?;

    GENERAL_STATE.with(|st| {
        st.key_management
            .borrow_mut()
            .insert(owner_principal.to_text(), encrypted_key.clone());
    });

    Ok(encrypted_key)
}

#[query]
fn get_vetkey_for_user(user_id: UserId) -> Option<Vec<u8>> {
    GENERAL_STATE.with(|st| retrieve_vetkey_per_user(user_id, &st.key_management))
}

/*
    Legacy Vault Specific Endpoints
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

/* 
    New vault-specific update endpoints 
*/

#[update] 
fn vault_spreadsheet_sync(vault_id: Principal, update: Vec<u8>) {
    let user_id = msg_caller();
    GENERAL_STATE.with(|state| {
        _vault_spreadsheet_sync(
            user_id,
            vault_id,
            update,
            &state.spreadsheet_map,
        );
    });
}

#[update]
fn vault_spreadsheet_deletes(vault_id: Principal, update: Vec<u8>) {
    let user_id = msg_caller();
    GENERAL_STATE.with(|state| {
        _vault_spreadsheet_delete(user_id, vault_id, update, &state.spreadsheet_map);
    });
}

#[update]
fn vault_login_full_sync(vault_id: Principal, update: Vec<u8>) {
    let user_id = msg_caller();
    GENERAL_STATE.with(|state| {
        _login_full_sync(
            user_id,
            vault_id,
            update,
            &state.logins_columns,
            &state.logins_map,
        );
    });
}

#[update]
fn vault_login_metadata_sync(vault_id: Principal, update: Vec<u8>) {
    let user_id = msg_caller();
    GENERAL_STATE.with(|state| {
        _login_metadata_sync(user_id, vault_id, update, &state.logins_columns, &state.logins_map);
    });
}

#[update]
fn vault_login_metadata_delete(vault_id: Principal, update: Vec<u8>) {
    let user_id = msg_caller();
    GENERAL_STATE.with(|state| {
        _login_metadata_delete(user_id, vault_id, update, &state.logins_columns, &state.logins_map);
    });
}

#[update]
fn vault_login_data_sync(vault_id: Principal, update: Vec<u8>) {
    let user_id = msg_caller();
    GENERAL_STATE.with(|state| {
        _login_data_sync(user_id, vault_id, update, &state.logins_map);
    });
}

#[update]
fn vault_login_data_deletes(vault_id: Principal, update: Vec<u8>) {
    let user_id = msg_caller();
    GENERAL_STATE.with(|state| {
        _login_data_deletes(user_id, vault_id, update, &state.logins_map);
    });
}

#[update]
fn global_sync(vault_id: Principal, update: Vec<u8>) {
    let user_id = msg_caller();
    GENERAL_STATE.with(|state| {
        _global_sync(
            user_id,
            vault_id,
            update,
            &state.logins_columns,
            &state.logins_map,
            &state.spreadsheet_map,
        );
    });
}

/* 
    New vault-specific query endpoints
*/

#[query]
fn get_spreadsheet(vault_id: Principal) -> vault_core::api::dev_api::Spreadsheet {
    let user_id = msg_caller();
    GENERAL_STATE.with(|state| {
        vault_core::api::dev_api::_get_spreadsheet(user_id, vault_id, &state.spreadsheet_map)
    })
}

ic_cdk::export_candid!();
