use soroban_sdk::{Address, Env, String};
use crate::storage;

const LIFETIME_THRESHOLD: u32 = 120960; // 1 semana em ledgers

pub fn init(env: Env, admin: Address, decimal: u32, name: String, symbol: String) {
    if storage::has_admin(&env) {
        panic!("already initialized");
    }

    env.storage().instance().extend_ttl(LIFETIME_THRESHOLD, LIFETIME_THRESHOLD);

    storage::set_admin(&env, &admin);
    storage::set_decimal(&env, &decimal);
    storage::set_name(&env, &name);
    storage::set_symbol(&env, &symbol);
}

pub fn mint(env: Env, to: Address, amount: i128) {
    let admin = storage::get_admin(&env);
    admin.require_auth();

    let balance = storage::get_balance(&env, &to);
    storage::set_balance(&env, &to, &(balance + amount));
}