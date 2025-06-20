#!/bin/bash
echo "🚀 Running ZHTP Installer in Development Mode..."
echo

echo "📦 Installing Tauri CLI (if not already installed)..."
cargo install tauri-cli --version "^2.0.0"

echo "🏃 Starting development server..."
cargo tauri dev
