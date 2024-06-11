// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./MyToken.sol";
import "./LPToken.sol";
import "../node_modules/@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../node_modules/@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

// fully functional liquidity pool that allows users to add liquidity, remove liquidity, and perform token swaps based on the x*y=k constant product formula
contract LiquidityPool {
    using SafeERC20 for IERC20;

    IERC20 public tokenA;
    IERC20 public tokenB;
    LPToken public lpToken;

    uint256 public reserveA;
    uint256 public reserveB;

    event LiquidityAdded(uint256 amountA, uint256 amountB, uint256 liquidity);
    event ReservesUpdated(uint256 reserveA, uint256 reserveB);

    constructor(address _tokenA, address _tokenB) {
        tokenA = IERC20(_tokenA);
        tokenB = IERC20(_tokenB);
        lpToken = new LPToken("Liquidity Pool Token", "LPT");
    }

    function balanceOf(address account) public view returns (uint256) {
        return lpToken.balanceOf(account);
    }

    // allows users to add liquidity to the pool by depositing an equal value of Token A and Token B
    function addLiquidity(uint256 amountA, uint256 amountB) external {
        require(amountA > 0 && amountB > 0, "Amounts must be greater than zero");

        uint256 lpTokenSupply = lpToken.totalSupply();
        uint256 liquidity;

        // calculates amount of LP tokens to mint based on the ratio of provided liquidity to existing liquidity
        if (lpTokenSupply == 0) {
            liquidity = sqrt(amountA * amountB);
        } else {
            liquidity = min(amountA * lpTokenSupply / reserveA, amountB * lpTokenSupply / reserveB);
        }

        require(liquidity > 0, "Insufficient liquidity minted");

        // update reserves
        reserveA += amountA;
        reserveB += amountB;

        // transfer tokens to the pool
        tokenA.safeTransferFrom(msg.sender, address(this), amountA);
        tokenB.safeTransferFrom(msg.sender, address(this), amountB);

        // mint LP tokens to the provider
        lpToken.mint(msg.sender, liquidity);

        emit LiquidityAdded(amountA, amountB, liquidity);
        emit ReservesUpdated(reserveA, reserveB);
    }

    // allows users to remove liquidity from the pool by burning LP tokens and receiving their share of the pool's reserves
    function removeLiquidity(uint256 liquidity) external {
        require(liquidity > 0, "Liquidity must be greater than zero");

        uint256 lpTokenSupply = lpToken.totalSupply();

        uint256 amountA = liquidity * reserveA / lpTokenSupply;
        uint256 amountB = liquidity * reserveB / lpTokenSupply;

        require(amountA > 0 && amountB > 0, "Insufficient liquidity burned");

        reserveA -= amountA;
        reserveB -= amountB;

        // transfer tokens back to the provider
        tokenA.safeTransfer(msg.sender, amountA);
        tokenB.safeTransfer(msg.sender, amountB);

        // burn LP tokens from the provider
        lpToken.burn(msg.sender, liquidity);

        emit ReservesUpdated(reserveA, reserveB);
    }

    // allows users to swap one token for another based on the constant product formula
    function swap(address tokenIn, uint256 amountIn) external {
        require(amountIn > 0, "Amount must be greater than zero");

        bool isTokenA = tokenIn == address(tokenA);
        bool isTokenB = tokenIn == address(tokenB);

        require(isTokenA || isTokenB, "Invalid token address");

        (uint256 reserveIn, uint256 reserveOut, IERC20 tokenOut) = isTokenA
            ? (reserveA, reserveB, tokenB)
            : (reserveB, reserveA, tokenA);

        uint256 amountOut = getAmountOut(amountIn, reserveIn, reserveOut);

        require(amountOut > 0, "Insufficient output amount");

        // update reserves
        reserveIn += amountIn;
        reserveOut -= amountOut;

        if (isTokenA) {
            reserveA = reserveIn;
            reserveB = reserveOut;
        } else {
            reserveA = reserveOut;
            reserveB = reserveIn;
        }

        // transfer tokens
        IERC20(tokenIn).safeTransferFrom(msg.sender, address(this), amountIn);
        tokenOut.safeTransfer(msg.sender, amountOut);

        emit ReservesUpdated(reserveA, reserveB);
    }

    // calculate the amount of Token B received for a specific amount of Token A in a swap (and vice versa)
    function getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut) public pure returns (uint256) {
        require(amountIn > 0, "Insufficient input amount");
        require(reserveIn > 0 && reserveOut > 0, "Insufficient liquidity");

        // takes into account a 0.3% fee
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = (reserveIn * 1000) + amountInWithFee;

        return numerator / denominator;
    }

    // calculate the amount of Token A required to receive a specific amount of Token B in a swap (and vice versa)
    function getAmountIn(uint256 amountOut, uint256 reserveIn, uint256 reserveOut) public pure returns (uint256) {
        require(amountOut > 0, "Insufficient output amount");
        require(reserveIn > 0 && reserveOut > 0, "Insufficient liquidity");

        uint256 numerator = reserveIn * amountOut * 1000;
        uint256 denominator = (reserveOut - amountOut) * 997;

        return (numerator / denominator) + 1;
    }

    // retrieve the current reserves of Token A and Token B
    function getReserves() external view returns (uint256, uint256) {
        return (reserveA, reserveB);
    }

    function sqrt(uint256 y) internal pure returns (uint256 z) {
        if (y > 3) {
            z = y;
            uint256 x = y / 2 + 1;
            while (x < z) {
                z = x;
                x = (y / x + x) / 2;
            }
        } else if (y != 0) {
            z = 1;
        }
    }

    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }
}