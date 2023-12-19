// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../lib/g3m/G3mLib.sol";

import "../interfaces/ICore.sol";
/**
 * @notice Geometric Mean Market Maker.
 */

contract G3m {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    ICore public core;
    uint256 public swapFee;
    Parameters public __slot__;

    uint256 private lastWeightX;
    uint256 private lastWeightXSync;
    uint256 private targetWeightX;
    uint256 private weightXUpdateEnd;
    uint256 private weightXUpdatePerSecond;

    modifier onlyCore() {
        // require(msg.sender == address(core), "only core");
        _;
    }

    /// @dev Returns the original parameters that were used to initialize the pool.
    function staticSlot() public view returns (Parameters memory) {
        return __slot__;
    }

    /// @dev Slot holds out parameters, these return the dyanmic parameters.
    function dynamicSlot() public view returns (Parameters memory params) {
        (params.wx, params.wy) = (weightX(), weightY());
    }

    function getReservesAndLiquidity()
        public
        view
        returns (uint256, uint256, uint256)
    {
        return core.getReservesAndLiquidity();
    }

    function _syncDynamicSlot() internal {
        Parameters memory params = staticSlot();

        targetWeightX = params.wx;
        lastWeightX = params.wx;
        weightXUpdateEnd = block.timestamp;
        lastWeightXSync = block.timestamp;
    }

    /// @dev Computes the result of the tradingFunction().
    function computeSwapConstant(bytes memory data)
        public
        view
        returns (int256)
    {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        return tradingFunction({ rx: rx, ry: ry, L: L, params: dynamicSlot() });
    }

    /// @dev Decodes and validates pool initialization parameters.
    /// Sets the `slot` state variable.
    function init(bytes calldata data)
        public
        onlyCore
        returns (
            bool valid,
            int256 invariant,
            uint256 rx,
            uint256 ry,
            uint256 L
        )
    {
        (rx, ry, L, __slot__) =
            abi.decode(data, (uint256, uint256, uint256, Parameters));

        _syncDynamicSlot();

        invariant =
            tradingFunction({ rx: rx, ry: ry, L: L, params: dynamicSlot() });

        // todo: should the be EXACTLY 0? just positive? within an epsilon?
        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @dev Reverts if the caller is not a contract with the Core interface.
    function validate(bytes memory data)
        public
        view
        onlyCore
        returns (
            bool valid,
            int256 invariant,
            int256 liquidityDelta,
            uint256 nextRx,
            uint256 nextRy,
            uint256 nextL
        )
    {
        (uint256 startRx, uint256 startRy, uint256 startL) =
            getReservesAndLiquidity();

        (nextRx, nextRy, nextL) = abi.decode(data, (uint256, uint256, uint256));

        uint256 minLiquidityDelta;
        uint256 amountIn;
        uint256 fees;
        if (nextRx > startRx) {
            amountIn = nextRx - startRx;
            fees = amountIn.mulWadUp(swapFee);
            minLiquidityDelta += fees.mulWadUp(startL).divWadUp(startRx);
        } else if (nextRy > startRy) {
            amountIn = nextRy - startRy;
            fees = amountIn.mulWadUp(swapFee);
            minLiquidityDelta += fees.mulWadUp(startL).divWadUp(startRy);
        } else {
            revert("invalid swap: inputs x and y have the same sign!");
        }

        liquidityDelta = int256(nextL) - int256(startL);

        invariant = tradingFunction({
            rx: nextRx,
            ry: nextRy,
            L: nextL,
            params: dynamicSlot()
        });

        bool validSwapConstant = -(EPSILON) < invariant && invariant < EPSILON;

        valid = validSwapConstant && liquidityDelta >= int256(minLiquidityDelta);
    }

    function weightX() public view returns (uint256) {
        if (block.timestamp >= weightXUpdateEnd) {
            return targetWeightX;
        }

        uint256 timeElapsed = block.timestamp - lastWeightXSync;
        uint256 weightXDelta = timeElapsed * weightXUpdatePerSecond;

        if (lastWeightX > targetWeightX) {
            return lastWeightX - weightXDelta;
        } else {
            return lastWeightX + weightXDelta;
        }
    }

    function weightY() public view returns (uint256) {
        return 1 ether - weightX();
    }
}