const MyToken = artifacts.require("MyToken");
const LiquidityPool = artifacts.require("LiquidityPool");

contract("LiquidityPool", (accounts) => {
    let tokenA;
    let tokenB;
    let liquidityPool;

    before(async () => {
        tokenA = await MyToken.new("Token A", "TKA");
        tokenB = await MyToken.new("Token B", "TKB");
        liquidityPool = await LiquidityPool.new(tokenA.address, tokenB.address);
    });

    it("should add liquidity", async () => {
        // Mint tokens to the first account
        await tokenA.mint(accounts[0], web3.utils.toWei("1000"));
        await tokenB.mint(accounts[0], web3.utils.toWei("1000"));

        // Approve the liquidity pool to spend tokens
        await tokenA.approve(liquidityPool.address, web3.utils.toWei("1000"));
        await tokenB.approve(liquidityPool.address, web3.utils.toWei("1000"));

        // Add liquidity
        await liquidityPool.addLiquidity(web3.utils.toWei("100"), web3.utils.toWei("100"));

        // Check the liquidity provider's LP token balance
        const lpTokenBalance = await liquidityPool.balanceOf(accounts[0]);
        assert.equal(lpTokenBalance.toString(), web3.utils.toWei("100"));
    });

    it("should swap tokens", async () => {
        // Mint tokens to the first account
        await tokenA.mint(accounts[0], web3.utils.toWei("1000"));

        // Approve the liquidity pool to spend tokens
        await tokenA.approve(liquidityPool.address, web3.utils.toWei("100"));

        // Get the initial token balances
        const initialTokenABalance = await tokenA.balanceOf(accounts[0]);
        const initialTokenBBalance = await tokenB.balanceOf(accounts[0]);

        // Swap Token A for Token B
        await liquidityPool.swap(tokenA.address, web3.utils.toWei("100"));

        // Get the updated token balances
        const finalTokenABalance = await tokenA.balanceOf(accounts[0]);
        const finalTokenBBalance = await tokenB.balanceOf(accounts[0]);

        // Check the token balance changes
        assert.equal(finalTokenABalance.toString(), initialTokenABalance.sub(web3.utils.toBN(web3.utils.toWei("100"))).toString());
        assert.notEqual(finalTokenBBalance.toString(), initialTokenBBalance.toString());
    });

    it("should remove liquidity", async () => {
        // Remove liquidity
        await liquidityPool.removeLiquidity(web3.utils.toWei("100"));

        // Check the account's token balances
        const tokenABalance = await tokenA.balanceOf(accounts[0]);
        const tokenBBalance = await tokenB.balanceOf(accounts[0]);
        assert.notEqual(tokenABalance.toString(), web3.utils.toWei("900"));
        assert.notEqual(tokenBBalance.toString(), web3.utils.toWei("0"));

        // Check the account's LP token balance
        const lpTokenBalance = await liquidityPool.balanceOf(accounts[0]);
        assert.equal(lpTokenBalance.toString(), web3.utils.toWei("0"));
    });
});