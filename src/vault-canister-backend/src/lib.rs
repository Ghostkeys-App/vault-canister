use candid::Principal;
use ic_cdk::{inspect_message, query, update};
use vault_core::{
    api::{
        key_api::{derive_vetkey, storage_user_of, GhostkeysVetKdArgs},
    },
    stable::{types::GeneralState, util::{_init_controllers, _inspect_message, maintain_status}},
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

#[inspect_message]
fn inspect_message() {
    let always_accept: Vec<String> = vec![
        "canister_init".to_string(), // needs to be reworked so only the
    ];
    // call common inspect
    GENERAL_STATE.with(|m| _inspect_message(&always_accept, &m.canister_owners))
}
#[update]
fn canister_init(user: Principal, controller: Principal) {
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

ic_cdk::export_candid!();
