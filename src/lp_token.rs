use web3::contract::{Contract, Options};
use web3::types::{Address, U256, H256};

pub struct LPToken {
    contract: Contract<web3::transports::Http>,
}

impl LPToken {
    pub fn new(web3: web3::Web3<web3::transports::Http>, address: Address) -> Self {
        let abi = include_str!("../abi/LPToken.abi");
        let contract = Contract::from_json(web3.eth(), address, abi.as_bytes()).unwrap();

        LPToken { contract }
    }

    pub async fn name(&self) -> web3::contract::Result<String> {
        self.contract.query("name", (), None, Options::default(), None).await
    }

    pub async fn symbol(&self) -> web3::contract::Result<String> {
        self.contract.query("symbol", (), None, Options::default(), None).await
    }

    pub async fn decimals(&self) -> web3::contract::Result<u8> {
        self.contract.query("decimals", (), None, Options::default(), None).await
    }

    pub async fn total_supply(&self) -> web3::contract::Result<U256> {
        self.contract.query("totalSupply", (), None, Options::default(), None).await
    }

    pub async fn balance_of(&self, account: Address) -> web3::contract::Result<U256> {
        self.contract.query("balanceOf", account, None, Options::default(), None).await
    }

    pub async fn allowance(&self, owner: Address, spender: Address) -> web3::contract::Result<U256> {
        self.contract.query("allowance", (owner, spender), None, Options::default(), None).await
    }

    pub async fn owner(&self) -> web3::contract::Result<Address> {
        self.contract.query("owner", (), None, Options::default(), None).await
    }

    pub async fn transfer(&self, to: Address, value: U256, from: Address) -> web3::contract::Result<H256> {
        let options = Options::default();
        self.contract.call("transfer", (to, value), from, options).await
    }

    pub async fn approve(&self, spender: Address, value: U256, from: Address) -> web3::contract::Result<H256> {
        let options = Options::default();
        self.contract.call("approve", (spender, value), from, options).await
    }

    pub async fn transfer_from(&self, from: Address, to: Address, value: U256, sender: Address) -> web3::contract::Result<H256> {
        let options = Options::default();
        self.contract.call("transferFrom", (from, to, value), sender, options).await
    }

    pub async fn mint(&self, to: Address, amount: U256, from: Address) -> web3::contract::Result<H256> {
        let options = Options::default();
        self.contract.call("mint", (to, amount), from, options).await
    }

    pub async fn burn(&self, from: Address, amount: U256) -> web3::contract::Result<H256> {
        let options = Options::default();
        self.contract.call("burn", (from, amount), from, options).await
    }

    pub async fn renounce_ownership(&self, from: Address) -> web3::contract::Result<H256> {
        let options = Options::default();
        self.contract.call("renounceOwnership", (), from, options).await
    }

    pub async fn transfer_ownership(&self, new_owner: Address, from: Address) -> web3::contract::Result<H256> {
        let options = Options::default();
        self.contract.call("transferOwnership", new_owner, from, options).await
    }
}
