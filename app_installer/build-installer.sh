#!/bin/bash
echo "🔨 Building ZHTP Installer for Linux..."
echo

echo "📦 Installing Tauri CLI..."
cargo install tauri-cli --version "^2.0.0"

echo "🔨 Building installer executable..."
cargo tauri build

echo
echo "✅ Build complete!"
echo "📁 Installer location: target/release/bundle/"
echo
echo "🚀 Run the installer:"
echo "   DEB: sudo dpkg -i target/release/bundle/deb/zhtp-installer_1.0.0_amd64.deb"
echo "   AppImage: ./target/release/bundle/appimage/zhtp-installer_1.0.0_amd64.AppImage"
