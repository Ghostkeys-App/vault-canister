use ic_cdk::{api::canister_liquid_cycle_balance, call::Call};
use crate::stable::types::CanisterOwnersState;
use candid::{Principal, Decode};


// Define constants
// Threshold at which the canister should request a top-up.
// This must be a value sufficient to serve a request and continue storing data until the top-up is received.
pub const MIN_CYCLES_BALANCE: u128 = 1_000_000_000;

/// Maintains the status of the canister, performing any tasks needed to keep it operational.
/// Every update call should trigger this function.
pub fn maintain_status(canister_owners: &CanisterOwnersState) {
    let can_cycles = canister_liquid_cycle_balance();

    if can_cycles < MIN_CYCLES_BALANCE {
        // get the owning principal (currently factory can)
        let owner = canister_owners.borrow().controller;
        // call the top_up method on the canister
        let _result = Call::unbounded_wait(
            owner,
            "top_up",
        );
    }
}

pub fn init_controllers(arg: Vec<u8>, canister_owners: &CanisterOwnersState) {
    let (user, controller): (Principal, Principal) =
    Decode!(&arg, (Principal, Principal)).expect("Failed to decode canister init arguments");

    ic_cdk::println!("Canister initialized with user: {}, controller: {}", user, controller);
    canister_owners.borrow_mut().user.push(user);
    canister_owners.borrow_mut().controller = controller;
}