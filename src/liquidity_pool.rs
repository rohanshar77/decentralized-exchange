use web3::contract::{Contract, Options};
use web3::types::{Address, U256, H256};

pub struct LiquidityPool {
    contract: Contract<web3::transports::Http>,
    address: Address,
}

impl LiquidityPool {
    pub fn new(web3: web3::Web3<web3::transports::Http>, address: Address) -> Self {
        let abi = include_str!("../abi/LiquidityPool.abi");
        let contract = Contract::from_json(web3.eth(), address, abi.as_bytes()).unwrap();

        LiquidityPool { contract, address }
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub async fn token_a(&self) -> web3::contract::Result<Address> {
        self.contract.query("tokenA", (), None, Options::default(), None).await
    }

    pub async fn token_b(&self) -> web3::contract::Result<Address> {
        self.contract.query("tokenB", (), None, Options::default(), None).await
    }

    pub async fn lp_token(&self) -> web3::contract::Result<Address> {
        self.contract.query("lpToken", (), None, Options::default(), None).await
    }

    pub async fn reserve_a(&self) -> web3::contract::Result<U256> {
        self.contract.query("reserveA", (), None, Options::default(), None).await
    }

    pub async fn reserve_b(&self) -> web3::contract::Result<U256> {
        self.contract.query("reserveB", (), None, Options::default(), None).await
    }

    pub async fn balance_of(&self, account: Address) -> web3::contract::Result<U256> {
        self.contract.query("balanceOf", account, None, Options::default(), None).await
    }

    pub async fn add_liquidity(&self, amount_a: U256, amount_b: U256, from: Address) -> web3::contract::Result<H256> {
        let options = Options::default();
        self.contract.call("addLiquidity", (amount_a, amount_b), from, options).await
    }

    pub async fn remove_liquidity(&self, lp_amount: U256, from: Address) -> web3::contract::Result<H256> {
        let options = Options::default();
        self.contract.call("removeLiquidity", lp_amount, from, options).await
    }

    pub async fn get_price(&self) -> web3::contract::Result<U256> {
        self.contract.query("getPrice", (), None, Options::default(), None).await
    }

    pub async fn swap(&self, token_in: Address, amount_in: U256, from: Address) -> web3::contract::Result<H256> {
        let options = Options::default();
        self.contract.call("swap", (token_in, amount_in), from, options).await
    }

    pub async fn get_amount_out(&self, amount_in: U256, reserve_in: U256, reserve_out: U256) -> web3::contract::Result<U256> {
        self.contract.query("getAmountOut", (amount_in, reserve_in, reserve_out), None, Options::default(), None).await
    }

    pub async fn get_amount_in(&self, amount_out: U256, reserve_in: U256, reserve_out: U256) -> web3::contract::Result<U256> {
        self.contract.query("getAmountIn", (amount_out, reserve_in, reserve_out), None, Options::default(), None).await
    }

    pub async fn get_reserves(&self) -> web3::contract::Result<(U256, U256)> {
        self.contract.query("getReserves", (), None, Options::default(), None).await
    }
}
