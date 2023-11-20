// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../utils/G3MTest.t.sol";

contract G3MComputeSpotPrice is G3MTest {
    function test_g3m_computeSpotPrice_ComputesSpotPrice() public {
        UD60x18 reserveX = convert(833_000 ether);
        UD60x18 reserveY = convert(500 ether);
        UD60x18 weightX = ud(0.5 ether);
        UD60x18 weightY = ud(0.5 ether);

        uint256 spotPrice =
            computeSpotPrice(reserveY, weightY, reserveX, weightX);
        assertEq(spotPrice, 1666 ether);
    }
}