use clap::{App, Arg, Subcommand};
use tokio;
use web3::types::{Address, U256};
use std::str::FromStr;

mod lib;
use lib::LiquidityPoolWrapper;

#[tokio::main]
async fn main() {
    let matches = App::new("AMM DEX CLI")
        .version("1.0")
        .about("Interacts with AMM DEX smart contracts")
        .subcommand(Subcommand::with_name("add_liquidity")
            .about("Add liquidity to the pool")
            .arg(Arg::new("amount_a")
                .help("Amount of Token A to add")
                .required(true))
            .arg(Arg::new("amount_b")
                .help("Amount of Token B to add")
                .required(true))
            .arg(Arg::new("from")
                .help("Address from which to add liquidity")
                .required(true)))
        .subcommand(Subcommand::with_name("remove_liquidity")
            .about("Remove liquidity from the pool")
            .arg(Arg::new("lp_amount")
                .help("Amount of LP tokens to remove")
                .required(true))
            .arg(Arg::new("from")
                .help("Address from which to remove liquidity")
                .required(true)))
        .subcommand(Subcommand::with_name("swap_tokens")
            .about("Swap tokens in the pool")
            .arg(Arg::new("token_in")
                .help("Address of the token to swap in")
                .required(true))
            .arg(Arg::new("amount_in")
                .help("Amount of the token to swap in")
                .required(true))
            .arg(Arg::new("from")
                .help("Address from which to swap tokens")
                .required(true)))
        .subcommand(Subcommand::with_name("get_reserves")
            .about("Get the reserves of the liquidity pool"))
        .get_matches();

    let web3 = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(web3);
    let liquidity_pool_address = Address::from_str("0xYourLiquidityPoolContractAddress").unwrap();
    let liquidity_pool_wrapper = LiquidityPoolWrapper::new(web3.clone(), liquidity_pool_address).await;

    if let Some(matches) = matches.subcommand_matches("add_liquidity") {
        let amount_a: U256 = matches.value_of("amount_a").unwrap().parse().unwrap();
        let amount_b: U256 = matches.value_of("amount_b").unwrap().parse().unwrap();
        let from = Address::from_str(matches.value_of("from").unwrap()).unwrap();
        match liquidity_pool_wrapper.add_liquidity(amount_a, amount_b, from).await {
            Ok(tx) => println!("Liquidity added successfully: {:?}", tx),
            Err(e) => println!("Error adding liquidity: {:?}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("remove_liquidity") {
        let lp_amount: U256 = matches.value_of("lp_amount").unwrap().parse().unwrap();
        let from = Address::from_str(matches.value_of("from").unwrap()).unwrap();
        match liquidity_pool_wrapper.remove_liquidity(lp_amount, from).await {
            Ok(tx) => println!("Liquidity removed successfully: {:?}", tx),
            Err(e) => println!("Error removing liquidity: {:?}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("swap_tokens") {
        let token_in = Address::from_str(matches.value_of("token_in").unwrap()).unwrap();
        let amount_in: U256 = matches.value_of("amount_in").unwrap().parse().unwrap();
        let from = Address::from_str(matches.value_of("from").unwrap()).unwrap();
        match liquidity_pool_wrapper.swap_tokens(token_in, amount_in, from).await {
            Ok(tx) => println!("Tokens swapped successfully: {:?}", tx),
            Err(e) => println!("Error swapping tokens: {:?}", e),
        }
    } else if let Some(_) = matches.subcommand_matches("get_reserves") {
        match liquidity_pool_wrapper.get_reserves().await {
            Ok((reserve_a, reserve_b)) => println!("Reserves - Token A: {:?}, Token B: {:?}", reserve_a, reserve_b),
            Err(e) => println!("Error getting reserves: {:?}", e),
        }
    }
}
