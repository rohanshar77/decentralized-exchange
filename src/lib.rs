pub mod liquidity_pool;
pub mod lp_token;
pub mod my_token;

use web3::contract::Options;
use web3::types::{Address, U256, H256};
use web3::Web3;

pub struct LiquidityPoolWrapper {
    pub web3: Web3<web3::transports::Http>,
    pub liquidity_pool: liquidity_pool::LiquidityPool,
    pub lp_token: lp_token::LPToken,
    pub token_a: my_token::MyToken,
    pub token_b: my_token::MyToken,
}

impl LiquidityPoolWrapper {
    pub async fn new(web3: Web3<web3::transports::Http>, liquidity_pool_address: Address) -> Self {
        let liquidity_pool = liquidity_pool::LiquidityPool::new(web3.clone(), liquidity_pool_address);
        let lp_token_address = liquidity_pool.lp_token().await.unwrap();
        let lp_token = lp_token::LPToken::new(web3.clone(), lp_token_address);
        let token_a_address = liquidity_pool.token_a().await.unwrap();
        let token_a = my_token::MyToken::new(web3.clone(), token_a_address);
        let token_b_address = liquidity_pool.token_b().await.unwrap();
        let token_b = my_token::MyToken::new(web3.clone(), token_b_address);

        LiquidityPoolWrapper {
            web3,
            liquidity_pool,
            lp_token,
            token_a,
            token_b,
        }
    }

    pub async fn add_liquidity(&self, amount_a: U256, amount_b: U256, from: Address) -> web3::contract::Result<H256> {
        let options = Options::with(|options| options.gas = Some(10_000_000.into()));
        self.token_a.approve(self.liquidity_pool.address(), amount_a, from).await.unwrap();
        self.token_b.approve(self.liquidity_pool.address(), amount_b, from).await.unwrap();
        self.liquidity_pool.add_liquidity(amount_a, amount_b, from).await
    }

    pub async fn remove_liquidity(&self, lp_amount: U256, from: Address) -> web3::contract::Result<H256> {
        let options = Options::with(|options| options.gas = Some(10_000_000.into()));
        self.lp_token.approve(self.liquidity_pool.address(), lp_amount, from).await.unwrap();
        self.liquidity_pool.remove_liquidity(lp_amount, from).await
    }

    pub async fn swap_tokens(&self, token_in: Address, amount_in: U256, from: Address) -> web3::contract::Result<H256> {
        let options = Options::with(|options| options.gas = Some(10_000_000.into()));
        if token_in == self.token_a.address() {
            self.token_a.approve(self.liquidity_pool.address(), amount_in, from).await.unwrap();
        } else {
            self.token_b.approve(self.liquidity_pool.address(), amount_in, from).await.unwrap();
        }
        self.liquidity_pool.swap(token_in, amount_in, from).await
    }

    pub async fn get_reserves(&self) -> web3::contract::Result<(U256, U256)> {
        self.liquidity_pool.get_reserves().await
    }

    pub async fn get_token_balance(&self, token: Address, account: Address) -> web3::contract::Result<U256> {
        if token == self.token_a.address() {
            self.token_a.balance_of(account).await
        } else {
            self.token_b.balance_of(account).await
        }
    }
}
