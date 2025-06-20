#!/bin/bash
echo "🚀 Starting ZHTP Network..."
echo ""

# Start network service with output
echo "📡 Starting network service..."
echo "   (This may take a moment to compile...)"

# Run the network service (it includes the browser server)
cargo run --bin network-service --release &
NETWORK_PID=$!

echo "   Network service started (PID: $NETWORK_PID)"
echo "   Waiting for startup..."

# Wait for service to start
sleep 15

# Check if process is still running
if kill -0 $NETWORK_PID 2>/dev/null; then
    echo "✅ Network service is running"
    echo ""
    echo "🎉 ZHTP Network is running!"
    echo ""
    echo "🌐 Browser: http://localhost:4000/browser/"
    echo "🔧 API: http://localhost:4000/api/"
    echo ""
    echo "Press Ctrl+C to stop"
    
    trap 'echo ""; echo "Stopping..."; kill $NETWORK_PID 2>/dev/null; exit 0' INT
    wait $NETWORK_PID
else
    echo "❌ Network service failed to start"
    echo "   The process exited. Check the error messages above."
    exit 1
fi
