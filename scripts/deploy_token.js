const MyToken = artifacts.require("MyToken");
const LPToken = artifacts.require("LPToken");
const LiquidityPool = artifacts.require("LiquidityPool");

module.exports = async function (deployer) {
  await deployer.deploy(MyToken);
  const myTokenInstance = await MyToken.deployed();

  await deployer.deploy(LPToken);
  const lpTokenInstance = await LPToken.deployed();

  await deployer.deploy(LiquidityPool, myTokenInstance.address, lpTokenInstance.address);
  const liquidityPoolInstance = await LiquidityPool.deployed();

  console.log("MyToken Address:", myTokenInstance.address);
  console.log("LPToken Address:", lpTokenInstance.address);
  console.log("LiquidityPool Address:", liquidityPoolInstance.address);
};
