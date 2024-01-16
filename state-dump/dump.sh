#!/bin/bash

# This script is used to generate the initial setup state for the dispute game simulation.

# Default Anvil private key
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
# PUB_KEY="$(cast wallet addr $PRIVATE_KEY)"

# Anvil RPC
RPC_URL="http://localhost:8545"

# Start anvil in the background
anvil -a 1 &
# Capture the process ID
ANVIL_PID=$!

# Get the directory of this script
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Deploy the FPAC system using the Sepolia deploy configuration.
export IMPL_SALT="dispute_arbiter"
cd "$SCRIPT_DIR/monorepo/packages/contracts-bedrock/scripts/fpac" && \
    make cannon-prestate chain=sepolia && \
    forge install && \
    forge script FPACOPS.sol \
    --sig "deployFPAC()" \
    --private-key $PRIVATE_KEY \
    --rpc-url $RPC_URL \
    --broadcast \
    --chain-id 11155111 \
    -vvv

cp "$SCRIPT_DIR/monorepo/packages/contracts-bedrock/scripts/fpac/prestate-artifacts.tar.gz" "$SCRIPT_DIR/prestate-artifacts.tar.gz"

# cast rpc "anvil_dumpState"
cd "$SCRIPT_DIR" && \
    cast rpc anvil_dumpState --rpc-url http://localhost:8545 | jq -r | xxd -r -p | gzip -d > dump.json

# Kill anvil
kill $ANVIL_PID
