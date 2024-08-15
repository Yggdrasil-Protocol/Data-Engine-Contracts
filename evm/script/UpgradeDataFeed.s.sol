// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "../src/PriceFeed.sol"; // Make sure this path is correct

contract UpgradePriceFeed is Script {
    address constant PROXY_ADDRESS = 0x30B3731d5fE29E768Ab282dBF2c79D9A70776Ad0;

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(deployerPrivateKey);

        vm.startBroadcast(deployerPrivateKey);

        // Deploy new implementation
        PriceFeed newImplementation = new PriceFeed();
        console.log("New implementation deployed at:", address(newImplementation));

        // Cast the proxy to UUPSUpgradeable
        UUPSUpgradeable proxy = UUPSUpgradeable(PROXY_ADDRESS);

        // Upgrade the proxy to the new implementation
        proxy.upgradeToAndCall(address(newImplementation), "");
        console.log("Proxy upgraded to new implementation");

        vm.stopBroadcast();

        // Verify the upgrade
        address currentImplementation = address(uint160(uint256(vm.load(PROXY_ADDRESS, 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc))));
        console.log("Current implementation:", currentImplementation);
        require(currentImplementation == address(newImplementation), "Upgrade failed");

        console.log("Upgrade successful!");
    }
}