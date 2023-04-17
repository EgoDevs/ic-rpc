// ------------------
//
// **Here are ego dependencies, needed for ego injections**
//
// ------------------
// BTreeMap
use std::collections::BTreeMap;

// ego_types
use ego_types::registry::Registry;
use ego_types::user::User;

// ego_macros
use ego_macros::{inject_app_info_api, inject_ego_api};

// ic_cdk
use candid::candid_method;
use ic_cdk::caller;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;

// injected macros
use ego_example_mod::state::{
    app_info_get, app_info_post_upgrade, app_info_pre_upgrade, app_info_update, canister_add,
    canister_get_one, info_log_add, is_op, is_owner, is_user, log_list, op_add, owner_add,
    owner_add_with_name, owner_remove, owners_set, registry_post_upgrade, registry_pre_upgrade,
    user_add, user_remove, users_post_upgrade, users_pre_upgrade, users_set,
};

// ------------------
//
// **Project dependencies
//
// ------------------

// types
use ego_example_mod::types::ExampleState;

// ------------------
//
// ** injections
//
// ------------------
// injection ego apis
inject_ego_api!();
inject_app_info_api!();

#[warn(unused_must_use)]
#[init]
#[candid_method(init, rename = "init")]
fn canister_init() {
    let caller = caller();
    owner_add(caller);
}

#[derive(Clone, CandidType, Deserialize)]
pub struct StableState {
    pub example_state: ExampleState,
    users: Option<User>,
    registry: Option<Registry>,
    app_info: Option<AppInfo>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    info_log_add("enter omni_wallet pre_upgrade");

    // composite StableState
    let stable_state = StableState {
        example_state: ego_example_mod::state::pre_upgrade(),
        users: Some(users_pre_upgrade()),
        registry: Some(registry_pre_upgrade()),
        app_info: Some(app_info_pre_upgrade()),
    };

    ic_cdk::storage::stable_save((stable_state,)).expect("failed to save stable state");
}

#[post_upgrade]
pub fn post_upgrade() {
    info_log_add("enter omni_wallet post_upgrade");

    let (state,): (StableState,) =
        ic_cdk::storage::stable_restore().expect("failed to restore stable state");

    match state.users {
        None => {}
        Some(users) => {
            users_post_upgrade(users);
        }
    }

    match state.registry {
        None => {}
        Some(registry) => {
            registry_post_upgrade(registry);
        }
    }

    match state.app_info {
        None => {}
        Some(app_info) => {
            app_info_post_upgrade(app_info);
        }
    }
    ego_example_mod::state::post_upgrade(state.example_state);
}

#[update(name = "whoAmI", guard = "owner_guard")]
#[candid_method(update, rename = "whoAmI")]
pub fn who_am_i() -> Principal {
    ic_cdk::api::caller()
}
