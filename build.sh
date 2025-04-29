#!/usr/bin/env bash

set -euo pipefail

cd "${0%/*}/"
cargo build --release

# Define an array of binary names
binaries=("erc20")

# Loop through each binary and create the .polkavm file
for binary in "${binaries[@]}"; do
    echo "Creating ${binary}.polkavm..."
    polkatool link --strip --output "${binary}.polkavm" "target/riscv64emac-unknown-none-polkavm/release/${binary}"
done

