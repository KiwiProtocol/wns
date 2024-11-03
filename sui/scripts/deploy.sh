#!/bin/bash
# Deploy script for WNS on Sui network

# Compile the Move package
echo "Compiling WNS Move package..."
sui move build

# Publish package to Sui
echo "Publishing package to Sui..."
sui move publish --gas-budget 10000000
