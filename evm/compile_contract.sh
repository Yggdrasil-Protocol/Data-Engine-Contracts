#!/bin/bash

# Define the source file and output directory
SOURCE_FILE="src/PriceFeed.sol"
OUTPUT_DIR="output"

# Create the output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Get the absolute path of the project root
PROJECT_ROOT="$(pwd)"

# Define the full paths to the OpenZeppelin contracts
OZ_UPGRADEABLE="$PROJECT_ROOT/lib/openzeppelin-contracts-upgradeable/contracts"
OZ_CONTRACTS="$PROJECT_ROOT/lib/openzeppelin-contracts/contracts"

# Function to update import statements
update_imports() {
    echo "Updating import statements in $SOURCE_FILE"
    sed -i.bak '
        s|@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol|@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol|g
        s|@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol|@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol|g
        s|@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol|@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol|g
        s|@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol|@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol|g
    ' "$SOURCE_FILE"
    echo "Updated import statements:"
    grep "import" "$SOURCE_FILE"
}

# Function to compile the contract
compile_contract() {
    echo "Compiling $SOURCE_FILE"
    solc_output=$(solc --abi \
         --allow-paths .,lib,"$OZ_UPGRADEABLE","$OZ_CONTRACTS" \
         --base-path . \
         --include-path lib \
         --include-path "$OZ_UPGRADEABLE" \
         --include-path "$OZ_CONTRACTS" \
         --evm-version london \
         "$SOURCE_FILE" \
         -o "$OUTPUT_DIR" \
         --overwrite \
         --optimize \
         --optimize-runs 200 \
         remappings.txt 2>&1)
    
    echo "Solc Output:"
    echo "$solc_output"
    
    if [ $? -eq 0 ]; then
        echo "Compilation successful. ABI file generated in $OUTPUT_DIR"
    else
        echo "Compilation failed. Please check the error messages above."
    fi
}

# Main execution
echo "Project Root: $PROJECT_ROOT"
echo "OpenZeppelin Upgradeable Path: $OZ_UPGRADEABLE"
echo "OpenZeppelin Contracts Path: $OZ_CONTRACTS"

# Check if directories exist
[ -d "$OZ_UPGRADEABLE" ] && echo "OpenZeppelin Upgradeable directory exists" || echo "OpenZeppelin Upgradeable directory does not exist"
[ -d "$OZ_CONTRACTS" ] && echo "OpenZeppelin Contracts directory exists" || echo "OpenZeppelin Contracts directory does not exist"

# Check if source file exists
[ -f "$SOURCE_FILE" ] && echo "Source file exists: $SOURCE_FILE" || { echo "Source file does not exist: $SOURCE_FILE"; exit 1; }

# Print solc version
echo "Solc version:"
solc --version

# Update imports
update_imports

# Compile contract
compile_contract

# Print the contents of the output directory
echo "Contents of $OUTPUT_DIR:"
ls -l "$OUTPUT_DIR"

# Print the first few lines of the source file
echo "First 10 lines of $SOURCE_FILE:"
head -n 10 "$SOURCE_FILE"

# Print OpenZeppelin version
echo "OpenZeppelin Contracts Upgradeable version:"
if [ -f "$OZ_UPGRADEABLE/package.json" ]; then
    grep '"version":' "$OZ_UPGRADEABLE/package.json"
elif [ -d "$OZ_UPGRADEABLE/.git" ]; then
    (cd "$OZ_UPGRADEABLE" && git describe --tags)
else
    echo "Unable to determine OpenZeppelin version"
fi

# Print Solidity pragma from source file
echo "Solidity pragma in $SOURCE_FILE:"
grep "pragma solidity" "$SOURCE_FILE"