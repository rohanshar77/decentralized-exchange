use std::str::FromStr;
use web3::transports::Http;
use web3::types::{Address, H256, U256};

use decentralized_exchange::{LiquidityPoolWrapper, my_token::MyToken};

#[tokio::test]
async fn test_liquidity_pool() {
    // Set up web3 instance
    let transport = Http::new("http://127.0.0.1:8545").unwrap();
    let web3 = web3::Web3::new(transport);

    // Replace these with actual deployed contract addresses
    let my_token_address = Address::from_str("address_from_ganache").unwrap();
    let lp_token_address = Address::from_str("address_from_ganache").unwrap();
    let liquidity_pool_address = Address::from_str("address_from_ganache").unwrap();

    // Initialize contract structs
    let my_token = MyToken::new(web3.clone(), my_token_address);
    let liquidity_pool_future = LiquidityPoolWrapper::new(
        web3.clone(),
        liquidity_pool_address,
    );

    let liquidity_pool = liquidity_pool_future.await;

    // Test MyToken contract
    let name = my_token.name().await.unwrap();
    assert_eq!(name, "MyToken");

    let symbol = my_token.symbol().await.unwrap();
    assert_eq!(symbol, "MTK");

    let decimals = my_token.decimals().await.unwrap();
    assert_eq!(decimals, 18);

    let total_supply = my_token.total_supply().await.unwrap();
    assert_eq!(total_supply, U256::from(1_000_000u64) * U256::exp10(18));

    // Test LiquidityPool contract
    let amount_a = U256::from(100u64) * U256::exp10(18);
    let amount_b = U256::from(100u64) * U256::exp10(18);
    let user_address = Address::from_str("address_from_ganache").unwrap();

    // Check initial token balances
    let initial_balance_a = my_token.balance_of(user_address).await.unwrap();
    println!("Initial Balance of Token A: {}", initial_balance_a);
    assert!(initial_balance_a >= amount_a, "Insufficient balance for Token A");

    // Approve tokens for liquidity pool
    let approve_tx = my_token.approve(liquidity_pool_address, amount_a, user_address).await.unwrap();
    println!("Approve TX Hash: {:?}", approve_tx);

    // Add liquidity
    let tx_hash = liquidity_pool.add_liquidity(amount_a, amount_b, user_address).await.unwrap();
    println!("Add Liquidity TX Hash: {:?}", tx_hash);
    assert!(tx_hash != H256::zero());

    // Get reserves immediately after adding liquidity
    let (reserve_a, reserve_b) = liquidity_pool.get_reserves().await.unwrap();
    println!("Reserve A: {}", reserve_a);
    println!("Reserve B: {}", reserve_b);

    // Ensure reserves are updated correctly
    assert_eq!(reserve_a, amount_a, "Reserve A mismatch");
    assert_eq!(reserve_b, amount_b, "Reserve B mismatch");

    // Remove liquidity
    let lp_balance = liquidity_pool.lp_token.balance_of(user_address).await.unwrap();
    let tx_hash = liquidity_pool.remove_liquidity(lp_balance, user_address).await.unwrap();
    assert!(tx_hash != H256::zero());

    // Swap tokens
    let swap_amount = U256::from(10u64) * U256::exp10(18);
    let tx_hash = liquidity_pool.swap_tokens(my_token_address, swap_amount, user_address).await.unwrap();
    assert!(tx_hash != H256::zero());

    // Check token balance after swap
    let new_balance = my_token.balance_of(user_address).await.unwrap();
    assert!(new_balance > U256::from(0));
}
