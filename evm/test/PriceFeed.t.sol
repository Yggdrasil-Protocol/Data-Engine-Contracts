// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/PriceFeed.sol";
import "../src/PriceFeedProxy.sol";
import "../lib/openzeppelin-contracts-upgradeable/contracts/proxy/utils/UUPSUpgradeable.sol";

contract PriceFeedTest is Test {
    PriceFeed private priceFeed;
    PriceFeedProxy private proxy;
    PriceFeed private proxiedPriceFeed;

    address private owner = address(1);
    address private user = address(2);

    function setUp() public {
        // Deploy the implementation contract
        priceFeed = new PriceFeed();

        // Encode the initializer call
        bytes memory data = abi.encodeWithSignature("initialize(uint256)", 1 ether);

        // Deploy the proxy contract
        proxy = new PriceFeedProxy(address(priceFeed), data);

        // Interact with the contract through the proxy
        proxiedPriceFeed = PriceFeed(payable(address(proxy)));

        // Transfer ownership to the owner address
        proxiedPriceFeed.transferOwnership(owner);

        // Deal some ETH to the user for testing
        vm.deal(user, 10 ether);
    }

    function testInitialFeePerAsset() public {
        assertEq(proxiedPriceFeed.feePerAsset(), 1 ether);
    }

    function testSetFeePerAsset() public {
        vm.startPrank(owner);
        proxiedPriceFeed.setFeePerAsset(2 ether);
        assertEq(proxiedPriceFeed.feePerAsset(), 2 ether);
        vm.stopPrank();
    }

    function testSetFeePerAssetUnauthorized() public {
        vm.startPrank(user);
        vm.expectRevert("Ownable: caller is not the owner");
        proxiedPriceFeed.setFeePerAsset(2 ether);
        vm.stopPrank();
    }

    function testUpdatePrice() public {
        bytes32[] memory assets = new bytes32[](1);
        uint8[] memory decimals = new uint8[](1);
        uint256[] memory prices = new uint256[](1);

        assets[0] = keccak256("ETH");
        decimals[0] = 18;
        prices[0] = 2000 ether;

        vm.startPrank(owner);
        proxiedPriceFeed.updatePrice(assets, decimals, prices);

        vm.stopPrank();
    }

    function testUpdatePriceUnauthorized() public {
        bytes32[] memory assets = new bytes32[](1);
        uint8[] memory decimals = new uint8[](1);
        uint256[] memory prices = new uint256[](1);

        assets[0] = keccak256("ETH");
        decimals[0] = 18;
        prices[0] = 2000 ether;

        vm.startPrank(user);
        vm.expectRevert("Ownable: caller is not the owner");
        proxiedPriceFeed.updatePrice(assets, decimals, prices);
        vm.stopPrank();
    }

    function testRequestPrices() public {
        bytes32[] memory assets = new bytes32[](1);
        uint8[] memory decimals = new uint8[](1);
        uint256[] memory prices = new uint256[](1);

        assets[0] = keccak256("ETH");
        decimals[0] = 18;
        prices[0] = 2000 ether;

        vm.startPrank(owner);
        proxiedPriceFeed.updatePrice(assets, decimals, prices);
        vm.stopPrank();

        vm.deal(user, 10 ether);

        function(uint8[] memory, uint256[] memory) external callback = this.priceCallback;

        vm.startPrank(user);
        proxiedPriceFeed.requestPrices{value: 1 ether}(assets, callback);
        vm.stopPrank();
    }

    function priceCallback(uint8[] memory, uint256[] memory) external pure {}

    function testWithdraw() public {
        // Send ETH to the contract
        vm.deal(address(proxy), 10 ether);

        vm.startPrank(owner);
        uint256 balanceBefore = owner.balance;
        proxiedPriceFeed.withdraw();
        uint256 balanceAfter = owner.balance;
        assertEq(balanceAfter, balanceBefore + 10 ether);
        vm.stopPrank();
    }

    function testWithdrawUnauthorized() public {
        // Send ETH to the contract
        vm.deal(address(proxy), 10 ether);

        vm.startPrank(user);
        vm.expectRevert("Ownable: caller is not the owner");
        proxiedPriceFeed.withdraw();
        vm.stopPrank();
    }
}
