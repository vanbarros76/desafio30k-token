#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_init() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Desafio30kToken);
    let client = Desafio30kTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    
    client.init(
        &admin,
        &7,
        &String::from_str(&env, "Desafio30k"),
        &String::from_str(&env, "D30K"),
    );

    assert_eq!(client.decimals(), 7);
    assert_eq!(client.name(), String::from_str(&env, "Desafio30k"));
    assert_eq!(client.symbol(), String::from_str(&env, "D30K"));
}

#[test]
fn test_mint() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Desafio30kToken);
    let client = Desafio30kTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    // Initialize contract
    client.init(
        &admin,
        &7,
        &String::from_str(&env, "Desafio30k"),
        &String::from_str(&env, "D30K"),
    );

    // Mint tokens
    env.mock_all_auths();
    client.mint(&user, &1000);

    // Check balance
    assert_eq!(client.balance(&user), 1000);
}
