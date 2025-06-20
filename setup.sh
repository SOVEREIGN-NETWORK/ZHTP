#!/bin/bash

# ZHTP Universal Setup Script
# Works on Windows (Git Bash/WSL), Linux, and macOS

set -e  # Exit on any error

# Colors for output
if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    # Windows Git Bash - simpler output
    RED='[ERROR]'
    GREEN='[SUCCESS]'
    YELLOW='[WARNING]'
    BLUE='[INFO]'
    NC=''
else
    # Unix/Linux - colored output
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    NC='\033[0m'
fi

print_status() { echo -e "${BLUE} $1${NC}"; }
print_success() { echo -e "${GREEN} $1${NC}"; }
print_warning() { echo -e "${YELLOW} $1${NC}"; }
print_error() { echo -e "${RED} $1${NC}"; }

echo "=================================================="
echo "🚀 ZHTP Universal Setup Script"
echo "Zero-Knowledge Hidden Transport Protocol"
echo "=================================================="
echo ""

# Detect operating system
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
        OS="windows"
    else
        print_warning "Unknown OS: $OSTYPE. Assuming Linux-like..."
        OS="linux"
    fi
    print_status "Detected OS: $OS"
}

# Install system dependencies
install_dependencies() {
    print_status "Installing system dependencies..."
    
    case $OS in
        "linux")
            if command -v apt-get &> /dev/null; then
                sudo apt-get update
                sudo apt-get install -y curl wget build-essential pkg-config libssl-dev python3 python3-pip jq git
            elif command -v yum &> /dev/null; then
                sudo yum update -y
                sudo yum install -y curl wget gcc gcc-c++ make pkgconfig openssl-devel python3 python3-pip jq git
            elif command -v pacman &> /dev/null; then
                sudo pacman -Sy
                sudo pacman -S --needed curl wget base-devel openssl python python-pip jq git
            else
                print_warning "Unknown package manager. Please install dependencies manually."
            fi
            ;;
        "macos")
            if ! command -v brew &> /dev/null; then
                print_status "Installing Homebrew..."
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            fi
            brew install curl wget python3 jq git
            ;;
        "windows")
            print_warning "On Windows, ensure you have:"
            print_warning "- Git for Windows (with Git Bash)"
            print_warning "- Python 3.x installed"
            print_warning "- Visual Studio Build Tools or Visual Studio with C++ support"
            ;;
    esac
    
    print_success "Dependencies installation completed"
}

# Install Rust
install_rust() {
    if ! command -v cargo &> /dev/null; then
        print_status "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        
        # Source Rust environment
        if [[ "$OS" == "windows" ]]; then
            export PATH="$HOME/.cargo/bin:$PATH"
        else
            source ~/.cargo/env
        fi
        
        print_success "Rust installed successfully"
    else
        print_success "Rust already installed"
    fi
    
    # Verify and update
    if command -v cargo &> /dev/null; then
        print_status "Updating Rust toolchain..."
        rustup update
        rustup target add wasm32-unknown-unknown
        print_success "Rust toolchain updated"
    else
        print_error "Rust installation failed. Please install manually."
        exit 1
    fi
}

# Build the project
build_project() {
    print_status "Building ZHTP network service..."
    
    # Set build environment
    export RUST_LOG=info
    
    # Build main project
    if cargo build --release; then
        print_success "ZHTP network service built successfully"
    else
        print_error "Build failed. Check the error messages above."
        exit 1
    fi
    
    # Build contracts
    if [[ -d "contracts" ]]; then
        print_status "Building smart contracts..."
        cd contracts
        if [[ -f "build.sh" ]]; then
            chmod +x build.sh
            ./build.sh
        elif [[ -f "build.bat" ]]; then
            ./build.bat
        else
            print_warning "No contract build script found"
        fi
        cd ..
        print_success "Smart contracts built"
    fi
}

# Create launch scripts
create_launch_scripts() {
    print_status "Creating launch scripts..."
    
    # Universal launch script
    cat > launch.sh << 'EOF'
#!/bin/bash
echo "🚀 Launching ZHTP Network..."

# Start network service
echo "📡 Starting network service..."
cargo run --bin network-service --release &
NETWORK_PID=$!

# Wait for service to start
sleep 5

# Check if service is running
if curl -s http://localhost:4444/api/status > /dev/null 2>&1; then
    echo "✅ Network service running (PID: $NETWORK_PID)"
else
    echo "❌ Network service failed to start"
    kill $NETWORK_PID 2>/dev/null
    exit 1
fi

# Start browser server
echo "🌐 Starting browser server..."
if command -v python3 &> /dev/null; then
    python3 -m http.server 4000 &
    BROWSER_PID=$!
elif command -v python &> /dev/null; then
    python -m http.server 4000 &
    BROWSER_PID=$!
else
    echo "❌ Python not found"
    kill $NETWORK_PID 2>/dev/null
    exit 1
fi

echo ""
echo "🎉 ZHTP Network is running!"
echo ""
echo "🌐 Browser: http://localhost:4000/browser/"
echo "🔧 API: http://localhost:4444/api/"
echo ""
echo "Press Ctrl+C to stop"

trap 'echo ""; echo "Stopping..."; kill $NETWORK_PID $BROWSER_PID 2>/dev/null; exit 0' INT
while true; do sleep 1; done
EOF

    # Windows batch file
    cat > launch.bat << 'EOF'
@echo off
echo 🚀 Launching ZHTP Network...

echo 📡 Starting network service...
start /B cargo run --bin network-service --release

timeout /t 5 > nul

echo 🌐 Starting browser server...
start /B python -m http.server 4000

echo.
echo 🎉 ZHTP Network is running!
echo.
echo 🌐 Browser: http://localhost:4000/browser/
echo 🔧 API: http://localhost:4444/api/
echo.
echo Press any key to stop...
pause > nul

taskkill /F /IM cargo.exe > nul 2>&1
taskkill /F /IM python.exe > nul 2>&1
EOF

    chmod +x launch.sh 2>/dev/null || true
    
    print_success "Launch scripts created"
}

# Main setup process
main() {
    detect_os
    install_dependencies
    install_rust
    build_project
    create_launch_scripts
    
    echo ""
    echo "=================================================="
    print_success "🎉 ZHTP Setup Complete!"
    echo "=================================================="
    echo ""
    echo "To start ZHTP:"
    if [[ "$OS" == "windows" ]]; then
        echo "  launch.bat"
    else
        echo "  ./launch.sh"
    fi
    echo ""
    echo "Then open: http://localhost:4000/browser/"
    echo ""
    print_success "Ready to launch the decentralized internet! 🚀"
}

# Run main setup
main
