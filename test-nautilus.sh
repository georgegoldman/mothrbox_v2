#!/bin/bash

echo "🚀 Starting Nautilus server..."
cd nautilus-enclave
cargo run --release &
SERVER_PID=$!

sleep 5

echo ""
echo "🧪 Testing health endpoint..."
curl -s http://localhost:8080/health | jq '.'

echo ""
echo "✅ Server is running!"
echo ""
echo "💡 Test encryption:"
echo "   echo 'Hello World!' | base64 > /tmp/test.b64"
echo "   curl -X POST http://localhost:8080/encrypt \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"file_data\":\"'\$(cat /tmp/test.b64)'\",\"password\":\"test\",\"algorithm\":\"aes\",\"filename\":\"test.txt\"}' | jq '.'"
echo ""
echo "Press Ctrl+C to stop (PID: $SERVER_PID)"
wait $SERVER_PID
