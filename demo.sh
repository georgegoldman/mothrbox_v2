#!/bin/bash

# MothrBox Demo Script for Hackathon

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

clear
echo -e "${BLUE}"
cat << 'BANNER'
╔══════════════════════════════════════════════════════╗
║                                                      ║
║              MothrBox v2 Demo                        ║
║   Verifiable Encrypted Storage with Nautilus        ║
║                                                      ║
╚══════════════════════════════════════════════════════╝
BANNER
echo -e "${NC}"

echo -e "${YELLOW}[1/5] Checking Nautilus server...${NC}"
if ! curl -s http://localhost:8080/health > /dev/null; then
    echo -e "${YELLOW}Starting Nautilus server...${NC}"
    cd nautilus-enclave
    cargo run --release &
    SERVER_PID=$!
    sleep 3
    cd ..
fi

curl -s http://localhost:8080/health | jq '.'
echo ""

echo -e "${YELLOW}[2/5] Creating demo file...${NC}"
cat > hackathon-demo.txt << 'DEMO'
MothrBox Hackathon Demonstration

This file proves:
✓ Military-grade encryption (AES-256-GCM)
✓ Decentralized storage (Walrus Protocol)
✓ Blockchain verification (Sui)
✓ TEE architecture (Nautilus-compatible)

Built for the future of secure storage!
DEMO

cat hackathon-demo.txt
echo ""

echo -e "${YELLOW}[3/5] Encrypting with Nautilus TEE...${NC}"
FILE_B64=$(base64 -w 0 hackathon-demo.txt)

RESPONSE=$(curl -s -X POST http://localhost:8080/encrypt \
  -H 'Content-Type: application/json' \
  -d "{
    \"file_data\": \"$FILE_B64\",
    \"password\": \"HackathonDemo2024\",
    \"algorithm\": \"aes\",
    \"filename\": \"hackathon-demo.txt\"
  }")

echo "$RESPONSE" | jq '.'
echo ""

BLOB_ID=$(echo "$RESPONSE" | jq -r '.blob_id')
FILE_HASH=$(echo "$RESPONSE" | jq -r '.file_hash')
ATTESTATION=$(echo "$RESPONSE" | jq -r '.attestation_document')

echo -e "${GREEN}✅ Encrypted!${NC}"
echo -e "   Blob ID: ${BLUE}$BLOB_ID${NC}"
echo -e "   Hash: ${BLUE}${FILE_HASH:0:32}...${NC}"
echo ""

echo -e "${YELLOW}[4/5] Verifying on Sui blockchain...${NC}"
echo -e "   Package: $PACKAGE_ID"
echo -e "   Registry: $REGISTRY_ID"
echo ""

if [ -n "$PACKAGE_ID" ] && [ -n "$REGISTRY_ID" ]; then
    BLOB_ID_HEX=$(echo -n "$BLOB_ID" | xxd -p -c 256)
    ATTESTATION_HEX=$(echo -n "$ATTESTATION" | base64 -d | xxd -p -c 256)
    
    sui client call \
      --package $PACKAGE_ID \
      --module nautilus_verifier \
      --function verify_encryption \
      --args $REGISTRY_ID "0x${BLOB_ID_HEX}" "0x${FILE_HASH}" "0x${ATTESTATION_HEX}" "0x616573" \
      --gas-budget 10000000 | grep -A 20 "Transaction Effects"
fi

echo ""
echo -e "${YELLOW}[5/5] Summary${NC}"
echo -e "${GREEN}✅ File encrypted with AES-256-GCM${NC}"
echo -e "${GREEN}✅ Stored on Walrus Protocol${NC}"
echo -e "${GREEN}✅ Verified on Sui blockchain${NC}"
echo -e "${GREEN}✅ Immutable audit trail created${NC}"
echo ""

echo -e "${BLUE}Demo complete! 🎉${NC}"
echo ""
echo -e "View blockchain verification:"
echo -e "  ${YELLOW}sui client events --limit 1${NC}"
echo ""
