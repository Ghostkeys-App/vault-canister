use vault_core::state::GeneralState;

thread_local! {
    static GENERAL_STATE: GeneralState = GeneralState::init();
}