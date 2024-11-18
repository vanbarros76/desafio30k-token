#![no_std]
mod admin;
mod storage;
mod token;

use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct Desafio30kToken;

#[contractimpl]
impl Desafio30kToken {
    pub fn init(env: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        admin::init(env, admin, decimal, name, symbol)
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        admin::mint(env, to, amount)
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        token::balance(env, id)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        token::transfer(env, from, to, amount)
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        token::approve(env, from, spender, amount, expiration_ledger)
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        token::allowance(env, from, spender)
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        token::transfer_from(env, spender, from, to, amount)
    }

    // Adicionando os métodos que faltavam
    pub fn decimals(env: Env) -> u32 {
        storage::get_decimal(&env)
    }

    pub fn name(env: Env) -> String {
        storage::get_name(&env)
    }

    pub fn symbol(env: Env) -> String {
        storage::get_symbol(&env)
    }
}
