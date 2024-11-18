use soroban_sdk::{Address, Env, String};

const ADMIN_KEY: &str = "admin";
const DECIMAL_KEY: &str = "decimal";
const NAME_KEY: &str = "name";
const SYMBOL_KEY: &str = "symbol";
const BALANCE_KEY: &str = "balance";
const ALLOWANCE_KEY: &str = "allowance";

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&ADMIN_KEY).unwrap()
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&ADMIN_KEY, admin);
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&ADMIN_KEY)
}

pub fn get_decimal(env: &Env) -> u32 {
    env.storage().instance().get(&DECIMAL_KEY).unwrap()
}

pub fn set_decimal(env: &Env, decimal: &u32) {
    env.storage().instance().set(&DECIMAL_KEY, decimal);
}

pub fn get_name(env: &Env) -> String {
    env.storage().instance().get(&NAME_KEY).unwrap()
}

pub fn set_name(env: &Env, name: &String) {
    env.storage().instance().set(&NAME_KEY, name);
}

pub fn get_symbol(env: &Env) -> String {
    env.storage().instance().get(&SYMBOL_KEY).unwrap()
}

pub fn set_symbol(env: &Env, symbol: &String) {
    env.storage().instance().set(&SYMBOL_KEY, symbol);
}

pub fn get_balance(env: &Env, addr: &Address) -> i128 {
    env.storage().persistent().get(&(BALANCE_KEY, addr)).unwrap_or(0)
}

pub fn set_balance(env: &Env, addr: &Address, amount: &i128) {
    env.storage().persistent().set(&(BALANCE_KEY, addr), amount);
}

pub fn get_allowance(env: &Env, from: &Address, spender: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&(ALLOWANCE_KEY, from, spender))
        .unwrap_or(0)
}

pub fn set_allowance(env: &Env, from: &Address, spender: &Address, amount: &i128) {
    env.storage()
        .persistent()
        .set(&(ALLOWANCE_KEY, from, spender), amount);
}