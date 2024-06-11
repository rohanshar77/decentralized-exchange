# AMM DEX Project

This is an open-source Automated Market Maker (AMM) Decentralized Exchange (DEX) project. The project involves integrating Solidity smart contracts with a Rust backend. The entire project runs locally and is interacted with via the CLI, using Ganache as the local Ethereum blockchain.

## What is an AMM DEX?

An Automated Market Maker (AMM) Decentralized Exchange (DEX) is a type of decentralized exchange protocol that relies on a mathematical formula to price assets. Instead of using an order book like traditional exchanges, AMMs allow digital assets to be traded in a permissionless and automatic way using liquidity pools.

### How AMM DEX Works

1. **Liquidity Providers (LPs)** add equal value of two tokens into a liquidity pool.
2. **Traders** can trade against this pool. The price of the tokens in the pool is determined by the ratio of the tokens in the pool.
3. LPs earn fees from trades that happen in their pool.

In an AMM, a liquidity pool consists of two tokens. When LPs add liquidity, they deposit an equal value of both tokens into the pool. Traders can then swap between these tokens. The core principle of the AMM is the constant product formula, represented as x * y = k, where x and y are the quantities of the two tokens, and k is a constant. This formula ensures that any trade will adjust the token quantities but keep their product constant, thereby determining the price.

### Benefits Over Traditional Exchanges

- **No Order Book**: AMMs do not require an order book, eliminating the need for buy and sell orders to match.
- **Continuous Liquidity**: Traders can always trade against the liquidity pool, ensuring continuous market activity.
- **Permissionless**: Anyone can provide liquidity and trade, promoting inclusivity and decentralization.


### Components

1. **Solidity Smart Contracts**:
   - `MyToken.sol`: ERC20 token implementation.
   - `LPToken.sol`: Liquidity Provider token implementation.
   - `LiquidityPool.sol`: Core liquidity pool logic.

2. **ABI Files**:
   - ABI files for the smart contracts to interact with the Rust backend.

3. **Rust Backend**:
   - `lib.rs`: Core library integrating the smart contracts.
   - `liquidity_pool.rs`, `lp_token.rs`, `my_token.rs`: Modules corresponding to the smart contracts.
   - `main.rs`: Entry point for the CLI to interact with the contracts.

## How to Run This Project

This project is designed to be run entirely locally using the Ganache CLI for a local Ethereum blockchain.

### Environment Setup

1. **Install Rust**
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustc --version
2. **Install Solidity Compiler (solc)**
   ```sh
   brew tap ethereum/ethereum
   brew install solidity
   solc --version
3. **Install Truffle Development Framework**
   ```sh
   npm install -g truffle
   truffle version
4. **Create Directory for Project**
   ```sh
   mkdir decentralized-exchange
   cd decentralized-exchange
5. **Initialize New Rust Project**
   ```sh
   cargo init
6. **Create Directory for Solidity Contracts**
   ```sh
   mkdir contracts
7. **Install Web3.js Library**
   ```sh
   npm install web3
8. **Install React and Create React App**
   ```sh
   npx create-react-app frontend
   cd frontend
9. **Install OpenZeppelin**
    ```sh
   npm install @openzeppelin/cli
   npm install @openzeppelin/contracts

### Running the Project

1. **Start Ganache CLI**
   ```sh
   ganache-cli
2. **Compile and Deploy Smart Contracts**
   Navigate to the `contracts` directory and run:
   ```sh
   truffle compile
   truffle migrate
3. **Generate ABI Files**
   Use the `ethabi-cli` tool to generate the ABI files:
   ```sh
   ethabi generate --input build/contracts/MyToken.json --output abi/MyToken.abi
   ethabi generate --input build/contracts/LPToken.json --output abi/LPToken.abi
   ethabi generate --input build/contracts/LiquidityPool.json --output abi/LiquidityPool.abi
4. **Run the Rust Backend**
   Navigate to the root directory of the project and run the Rust CLI:
   ```sh
   cargo run -- add_liquidity --amount_a 1000 --amount_b 2000 --from 0xYourAddress
   cargo run -- remove_liquidity --lp_amount 500 --from 0xYourAddress
   cargo run -- swap_tokens --token_in 0xTokenAAddress --amount_in 100 --from 0xYourAddress
   cargo run -- get_reserves
