#!/bin/bash

# Build script for MothrBox Rust CLI
# Builds the binary locally, then copies to Docker

set -e

echo "🦀 Building MothrBox Rust CLI..."

cd mothrbox_rs

# Build in release mode
echo "Building release binary..."
cargo build --release

# Check if build succeeded
if [ -f "target/release/mothrbox_rs" ]; then
    echo "✅ Build successful!"
    echo "📦 Binary location: mothrbox_rs/target/release/mothrbox_rs"
    
    # Copy to project root for easy access
    cp target/release/mothrbox_rs ../mothrbox
    chmod +x ../mothrbox
    
    echo "✅ Copied to: ./mothrbox"
    echo ""
    echo "Test it:"
    echo "  ./mothrbox --help"
    echo "  ./mothrbox aes encrypt test.txt test.enc password123"
else
    echo "❌ Build failed!"
    exit 1
fi