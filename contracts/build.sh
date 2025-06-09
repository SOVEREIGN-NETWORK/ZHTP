#!/bin/bash

# Build contract
wasm-pack build --target web

# Optimize WASM
wasm-opt -O4 -o token_opt.wasm pkg/zhtp_contracts_bg.wasm

# Copy to main project
cp token_opt.wasm ../token.wasm