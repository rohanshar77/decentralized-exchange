const MyToken = artifacts.require("MyToken");
const LPToken = artifacts.require("LPToken");
const LiquidityPool = artifacts.require("LiquidityPool");

module.exports = async function (deployer) {
  await deployer.deploy(MyToken, "My Token", "MTK");
  await deployer.deploy(LPToken, "Liquidity Pool Token", "LPT");

  const tokenA = await MyToken.deployed();
  const tokenB = await MyToken.new("Another Token", "ATK");

  await deployer.deploy(LiquidityPool, tokenA.address, tokenB.address);
};