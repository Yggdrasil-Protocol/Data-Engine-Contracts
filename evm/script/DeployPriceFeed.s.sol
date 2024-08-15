// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/PriceFeed.sol";
import "../src/PriceFeedProxy.sol";

contract DeployPriceFeed is Script {
    function run() external {
        // uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        // vm.startBroadcast(deployerPrivateKey);
        vm.startBroadcast();

        // Deploy the implementation contract
        PriceFeed priceFeedImplementation = new PriceFeed();

        // Set the initial fee per asset (adjust as needed)
        uint256 initialFeePerAsset = 0.0001 ether;

        // Prepare the initialization data
        bytes memory initData = abi.encodeWithSelector(
            PriceFeed.initialize.selector,
            initialFeePerAsset,
            msg.sender // Set the initial owner to the deployer
        );

        // Deploy the proxy contract
        PriceFeedProxy proxy = new PriceFeedProxy(
            address(priceFeedImplementation),
            initData
        );

        // The proxy address is now the main contract address to interact with
        address proxyAddress = address(proxy);

        console.log("PriceFeed implementation deployed at:", address(priceFeedImplementation));
        console.log("PriceFeed proxy deployed at:", proxyAddress);

        vm.stopBroadcast();
    }
}