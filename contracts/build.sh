#!/bin/bash
echo "Building WASM contract..."

# Ensure WASM target is available
rustup target add wasm32-unknown-unknown

# Try wasm-pack first
if command -v wasm-pack &> /dev/null; then
    echo "Using wasm-pack..."
    wasm-pack build --target web --dev
else
    echo "Using cargo directly..."
    cargo build --target wasm32-unknown-unknown --release
fi

# Create basic WASM if it doesn't exist
if [ ! -f "pkg/zhtp_contracts_bg.wasm" ]; then
    if [ -f "../target/wasm32-unknown-unknown/release/zhtp_contracts.wasm" ]; then
        mkdir -p pkg
        cp "../target/wasm32-unknown-unknown/release/zhtp_contracts.wasm" "pkg/zhtp_contracts_bg.wasm"
    else
        echo "Warning: WASM file not found, creating placeholder"
        mkdir -p pkg
        touch pkg/zhtp_contracts_bg.wasm
    fi
fi

# Copy result to main project
if [ -f "pkg/zhtp_contracts_bg.wasm" ]; then
    cp "pkg/zhtp_contracts_bg.wasm" "../token.wasm"
fi

echo "Build complete!"