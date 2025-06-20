#!/bin/bash
# ZHTP - Linux Startup Script
# Builds, runs ZHTP mainnet, and opens browser automatically!

set -e  # Exit on any error

# Colors for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_color() {
    printf "${1}${2}${NC}\n"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check if a port is in use
port_in_use() {
    lsof -Pi :$1 -sTCP:LISTEN -t >/dev/null 2>&1
}

# Function to open browser (cross-platform)
open_browser() {
    if command_exists xdg-open; then
        xdg-open "$1" >/dev/null 2>&1 &
    elif command_exists gnome-open; then
        gnome-open "$1" >/dev/null 2>&1 &
    elif command_exists open; then
        open "$1" >/dev/null 2>&1 &
    else
        print_color $YELLOW "Please open your browser manually to: $1"
    fi
}

# Function to cleanup processes on exit
cleanup() {
    print_color $YELLOW "\n🛑 Stopping ZHTP services..."
    
    # Kill background processes
    if [ ! -z "$MAINNET_PID" ] && kill -0 $MAINNET_PID 2>/dev/null; then
        kill $MAINNET_PID 2>/dev/null
        print_color $GREEN "✅ Stopped ZHTP Mainnet"
    fi
    
    if [ ! -z "$WEB_PID" ] && kill -0 $WEB_PID 2>/dev/null; then
        kill $WEB_PID 2>/dev/null
        print_color $GREEN "✅ Stopped ZHTP Web Service"
    fi
    
    # Kill any remaining cargo processes
    pkill -f "cargo run.*zhtp" 2>/dev/null || true
    
    print_color $GREEN "🏁 ZHTP services stopped successfully!"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

print_color $CYAN "🚀 ZHTP - Zero-Knowledge Hidden Transport"
print_color $CYAN "=========================================="
print_color $BLUE "Building and starting decentralized internet..."
echo

# Check prerequisites
print_color $YELLOW "🔍 Checking prerequisites..."

if ! command_exists cargo; then
    print_color $RED "❌ Cargo (Rust) is not installed!"
    print_color $YELLOW "Please install Rust from: https://rustup.rs/"
    exit 1
fi

if ! command_exists git; then
    print_color $RED "❌ Git is not installed!"
    print_color $YELLOW "Please install Git first."
    exit 1
fi

# Check for netcat or curl for port checking
if ! command_exists nc && ! command_exists curl; then
    print_color $YELLOW "⚠️ Neither netcat (nc) nor curl found."
    print_color $YELLOW "Installing curl for port checking..."
    if command_exists apt-get; then
        sudo apt-get update && sudo apt-get install -y curl
    elif command_exists yum; then
        sudo yum install -y curl
    elif command_exists pacman; then
        sudo pacman -S curl
    else
        print_color $YELLOW "Please install either 'nc' or 'curl' manually."
    fi
fi

print_color $GREEN "✅ Prerequisites check passed"
echo

# Check if ports are available
if port_in_use 4000; then
    print_color $RED "❌ Port 4000 is already in use!"
    print_color $YELLOW "Please stop the service using port 4000 and try again."
    exit 1
fi

print_color $GREEN "✅ Port availability check passed"
echo

# Build the project
print_color $YELLOW "🔨 Building ZHTP Mainnet..."
print_color $BLUE "   This may take a few minutes on first run..."

if cargo build --release; then
    print_color $GREEN "✅ Build successful!"
else
    print_color $RED "❌ Build failed!"
    print_color $YELLOW "Please check the error messages above."
    exit 1
fi

echo

# Start services
print_color $YELLOW "🌐 Starting ZHTP services..."

# Start ZHTP Mainnet Core
print_color $BLUE "   Starting ZHTP Mainnet Core..."
cargo run --example zhtp_mainnet_launch --release > /tmp/zhtp_mainnet.log 2>&1 &
MAINNET_PID=$!

# Start ZHTP Web Service
print_color $BLUE "   Starting ZHTP Web Service..."
cargo run --bin network-service --release > /tmp/zhtp_web.log 2>&1 &
WEB_PID=$!

# Wait for services to initialize
print_color $BLUE "   Waiting for services to initialize..."
sleep 5

# Wait for web service to be available on port 4000
print_color $BLUE "   Waiting for web service on port 4000..."
TIMEOUT=60
COUNTER=0
while [ $COUNTER -lt $TIMEOUT ]; do
    if nc -z localhost 4000 2>/dev/null || curl -s http://localhost:4000 >/dev/null 2>&1; then
        print_color $GREEN "   ✅ Web service is ready on port 4000"
        break
    fi
    sleep 1
    COUNTER=$((COUNTER + 1))
    if [ $((COUNTER % 10)) -eq 0 ]; then
        print_color $BLUE "   Still waiting for port 4000... ($COUNTER/$TIMEOUT seconds)"
    fi
done

if [ $COUNTER -eq $TIMEOUT ]; then
    print_color $RED "❌ Web service failed to start on port 4000 within $TIMEOUT seconds"
    print_color $YELLOW "Check log files for details:"
    print_color $YELLOW "   Mainnet: /tmp/zhtp_mainnet.log"
    print_color $YELLOW "   Web Service: /tmp/zhtp_web.log"
    cleanup
    exit 1
fi

# Check if services are running
SERVICES_OK=true

if ! kill -0 $MAINNET_PID 2>/dev/null; then
    print_color $RED "❌ ZHTP Mainnet failed to start"
    SERVICES_OK=false
fi

if ! kill -0 $WEB_PID 2>/dev/null; then
    print_color $RED "❌ ZHTP Web Service failed to start"
    SERVICES_OK=false
fi

if [ "$SERVICES_OK" = false ]; then
    print_color $YELLOW "Check log files for details:"
    print_color $YELLOW "   Mainnet: /tmp/zhtp_mainnet.log"
    print_color $YELLOW "   Web Service: /tmp/zhtp_web.log"
    cleanup
    exit 1
fi

print_color $GREEN "✅ All services started successfully!"
echo

# Open browser automatically
print_color $YELLOW "🌍 Opening ZHTP Browser Interface..."
sleep 2
open_browser "http://localhost:4000/browser/welcome.html"

# Display status information
print_color $GREEN "🎉 ZHTP MAINNET IS RUNNING!"
echo
print_color $CYAN "📊 Service Information:"
print_color $BLUE "   🌐 Browser Interface: http://localhost:4000/browser/welcome.html"
print_color $BLUE "   🔧 API Dashboard: http://localhost:4000/api/"
print_color $BLUE "   💰 Start earning ZHTP tokens!"
echo
print_color $PURPLE "🚀 Available DApps:"
print_color $PURPLE "   📰 news.zhtp - Decentralized news platform"
print_color $PURPLE "   👥 social.zhtp - Private social network"
print_color $PURPLE "   🛒 market.zhtp - P2P marketplace"
echo
print_color $CYAN "📋 Process Information:"
print_color $BLUE "   🔗 ZHTP Mainnet PID: $MAINNET_PID"
print_color $BLUE "   🌐 Web Service PID: $WEB_PID"
echo
print_color $YELLOW "📝 Log Files:"
print_color $BLUE "   📊 Mainnet: /tmp/zhtp_mainnet.log"
print_color $BLUE "   🌐 Web Service: /tmp/zhtp_web.log"
echo
print_color $GREEN "🎯 Your decentralized internet is ready!"
print_color $YELLOW "Press Ctrl+C to stop all services..."

# Keep the script running and wait for user to stop
while true; do
    sleep 1
    
    # Check if services are still running
    if ! kill -0 $MAINNET_PID 2>/dev/null || ! kill -0 $WEB_PID 2>/dev/null; then
        print_color $RED "❌ One or more services have stopped unexpectedly"
        print_color $YELLOW "Check the log files for details"
        cleanup
        exit 1
    fi
done
