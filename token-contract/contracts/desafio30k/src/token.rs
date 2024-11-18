use soroban_sdk::{Address, Env};
use crate::storage;

pub fn balance(env: Env, addr: Address) -> i128 {
    storage::get_balance(&env, &addr)
}

pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
    from.require_auth();

    let from_balance = storage::get_balance(&env, &from);
    if from_balance < amount {
        panic!("insufficient balance");
    }

    let to_balance = storage::get_balance(&env, &to);
    
    storage::set_balance(&env, &from, &(from_balance - amount));
    storage::set_balance(&env, &to, &(to_balance + amount));
}

pub fn approve(env: Env, from: Address, spender: Address, amount: i128, _expiration_ledger: u32) {
    from.require_auth();
    storage::set_allowance(&env, &from, &spender, &amount);
}

pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
    storage::get_allowance(&env, &from, &spender)
}

pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
    spender.require_auth();

    let allowance = storage::get_allowance(&env, &from, &spender);
    if allowance < amount {
        panic!("insufficient allowance");
    }

    let from_balance = storage::get_balance(&env, &from);
    if from_balance < amount {
        panic!("insufficient balance");
    }

    let to_balance = storage::get_balance(&env, &to);

    storage::set_allowance(&env, &from, &spender, &(allowance - amount));
    storage::set_balance(&env, &from, &(from_balance - amount));
    storage::set_balance(&env, &to, &(to_balance + amount));
}