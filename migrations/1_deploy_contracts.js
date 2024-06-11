const MyToken = artifacts.require("MyToken");
const LPToken = artifacts.require("LPToken");
const LiquidityPool = artifacts.require("LiquidityPool");

module.exports = async function (deployer, network, accounts) {
  // Deploy MyToken with name and symbol
  await deployer.deploy(MyToken, "MyToken", "MTK");
  const myTokenInstance = await MyToken.deployed();

  // Deploy LPToken with name and symbol
  await deployer.deploy(LPToken, "LPToken", "LPT");
  const lpTokenInstance = await LPToken.deployed();

  // Deploy LiquidityPool with MyToken and LPToken addresses
  await deployer.deploy(LiquidityPool, myTokenInstance.address, lpTokenInstance.address);
  const liquidityPoolInstance = await LiquidityPool.deployed();

  console.log("MyToken Address:", myTokenInstance.address);
  console.log("LPToken Address:", lpTokenInstance.address);
  console.log("LiquidityPool Address:", liquidityPoolInstance.address);
  console.log("User address:", accounts[0]);
};
