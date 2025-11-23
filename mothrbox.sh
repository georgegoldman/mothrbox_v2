#!/bin/bash

# MothrBox - Unified Encryption + Decentralized Storage System
# Interactive setup with .env configuration

set -e

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_info() { echo -e "${BLUE}ℹ${NC} $1"; }
print_success() { echo -e "${GREEN}✅${NC} $1"; }
print_warning() { echo -e "${YELLOW}⚠${NC} $1"; }
print_error() { echo -e "${RED}❌${NC} $1"; }

show_banner() {
    echo -e "${BLUE}"
    cat << 'EOF'
╔════════════════════════════════════════════════════════╗
║                     MOTHRBOX                           ║
║     Encrypted Decentralized Storage System             ║
╚════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
}

# Check prerequisites
check_prerequisites() {
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed!"
        echo "Install from: https://docs.docker.com/get-docker/"
        exit 1
    fi

    if command -v docker-compose &> /dev/null; then
        COMPOSE_CMD="docker-compose"
    elif docker compose version &> /dev/null 2>&1; then
        COMPOSE_CMD="docker compose"
    else
        print_error "docker-compose not found!"
        exit 1
    fi
}

# Interactive .env setup
setup_env() {
    print_info "Setting up MothrBox configuration..."
    echo ""

    ENV_DIR="mothrbox_ts"
    ENV_TEMPLATE="$ENV_DIR/.env.example"
    ENV_FILE="$ENV_DIR/.env"

    if [ ! -d "$ENV_DIR" ]; then
        print_error "Directory '$ENV_DIR' not found."
        exit 1
    fi

    # Handle existing .env
    if [ -f "$ENV_FILE" ]; then
        print_warning ".env file already exists at $ENV_FILE"
        read -p "Overwrite it? (yes/no): " overwrite
        if [ "$overwrite" = "yes" ] || [ "$overwrite" = "y" ]; then
            if [ -f "$ENV_TEMPLATE" ]; then
                cp "$ENV_TEMPLATE" "$ENV_FILE"
                print_success "Overwritten .env from template"
            else
                : > "$ENV_FILE"
                print_success "Created empty .env"
            fi
        else
            print_info "Keeping existing .env"
        fi
    else
        # no .env exists
        if [ -f "$ENV_TEMPLATE" ]; then
            cp "$ENV_TEMPLATE" "$ENV_FILE"
            print_success "Created .env from template"
        else
            : > "$ENV_FILE"
            print_success "Created empty .env"
        fi
    fi

    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "         SELECT SUI NETWORK"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "1. Testnet"
    echo "2. Mainnet"
    echo ""
    read -p "Choose network [1-2]: " netchoice

    case "$netchoice" in
        1)
            sed -i 's/^SUI_NETWORK=.*/SUI_NETWORK=testnet/' "$ENV_FILE" 2>/dev/null || echo "SUI_NETWORK=testnet" >> "$ENV_FILE"
            print_success "Set SUI_NETWORK=testnet"
            ;;
        2)
            sed -i 's/^SUI_NETWORK=.*/SUI_NETWORK=mainnet/' "$ENV_FILE" 2>/dev/null || echo "SUI_NETWORK=mainnet" >> "$ENV_FILE"
            print_success "Set SUI_NETWORK=mainnet"
            ;;
        *)
            print_warning "Invalid option, keeping existing SUI_NETWORK."
            ;;
    esac

    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "         ENTER PRIVATE KEY (HIDDEN)"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    # hidden key input
    read -s -p "Paste your Sui private key (hidden): " sui_key
    echo ""
    read -s -p "Re-enter your Sui private key (confirm): " sui_key_confirm
    echo ""

    if [ "$sui_key" != "$sui_key_confirm" ]; then
        print_error "Private keys do not match."
        exit 1
    fi

    if [[ "$sui_key" != suiprivkey1* ]]; then
        print_error "Invalid key — must start with 'suiprivkey1'"
        exit 1
    fi

    sed -i "s|^SUI_SECRET_KEY=.*|SUI_SECRET_KEY=$sui_key|" "$ENV_FILE" 2>/dev/null || echo "SUI_SECRET_KEY=$sui_key" >> "$ENV_FILE"
    print_success "Private key saved (hidden input)"

    echo ""
    print_success "✔ Configuration complete! $ENV_FILE is ready."
}



# Show help
show_help() {
    echo "USAGE:"
    echo "  ./mothrbox <command> [options]"
    echo ""
    echo "FIRST TIME:"
    echo "  setup              Interactive setup wizard"
    echo ""
    echo "SYSTEM:"
    echo "  start              Start MothrBox"
    echo "  stop               Stop MothrBox"
    echo "  status             Check status"
    echo "  logs               View logs"
    echo "  test               Run tests"
    echo ""
    echo "ENCRYPT:"
    echo "  encrypt <file> <pass>           Encrypt & upload"
    echo "  decrypt <id> <file> <pass>      Download & decrypt"
    echo "  keygen                          Generate ECC keys"
    echo ""
    echo "QUICK START:"
    echo "  ./mothrbox setup    # First time"
    echo "  ./mothrbox start    # Start system"
    echo "  ./mothrbox test     # Verify it works"
}

# Main script
show_banner
check_prerequisites

if [ $# -eq 0 ]; then
    show_help
    exit 0
fi

COMMAND=$1
shift

case $COMMAND in
    setup)
        setup_env
        
        read -p "Start MothrBox now? (yes/no): " start_now
        if [ "$start_now" = "yes" ] || [ "$start_now" = "y" ]; then
            print_info "Starting MothrBox..."
            $0 start
        else
            print_info "Setup complete! Start with: ./mothrbox start"
        fi
        ;;
    
    start)
        print_info "Starting MothrBox system..."
        
        # Check .env exists
        if [ ! -f "mothrbox_ts/.env" ]; then
            print_error "Configuration not found!"
            echo ""
            print_info "Run setup first: ./mothrbox setup"
            exit 1
        fi
        
        # Verify key is set
        if grep -q "your_sui_private_key_here" mothrbox_ts/.env; then
            print_error "Please configure your Sui private key"
            print_info "Run: ./mothrbox setup"
            exit 1
        fi
        
        # Show key type being used
        if grep -q "suiprivkey1qpvv2g070y4ewm3fam6yp7y8gq3w3y4c4a" mothrbox_ts/.env; then
            print_warning "Using test key (OK for testing)"
        else
            print_success "Using custom Sui key"
        fi
        
        mkdir -p data
        
        if ! docker images | grep -q "mothrbox"; then
            print_info "Building MothrBox (first time)..."
            $COMPOSE_CMD build
        fi
        
        print_info "Starting server..."
        $COMPOSE_CMD up -d mothrbox-server
        
        print_info "Waiting for server..."
        for i in {1..30}; do
            if $COMPOSE_CMD ps | grep -q "mothrbox-server.*Up"; then
                print_success "MothrBox is ready!"
                print_success "Server: http://localhost:8000"
                echo ""
                print_info "Try: ./mothrbox test"
                exit 0
            fi
            sleep 1
        done
        
        print_error "Server failed to start"
        print_info "Check logs: ./mothrbox logs"
        exit 1
        ;;
    
    stop)
        print_info "Stopping MothrBox..."
        $COMPOSE_CMD down
        print_success "Stopped"
        ;;
    
    status)
        if $COMPOSE_CMD ps | grep -q "mothrbox-server.*Up"; then
            print_success "Running: http://localhost:8000"
        else
            print_error "Not running"
        fi
        $COMPOSE_CMD ps
        ;;
    
    logs)
        $COMPOSE_CMD logs -f mothrbox-server
        ;;
    
    test)
        if ! $COMPOSE_CMD ps | grep -q "mothrbox-server.*Up"; then
            print_error "MothrBox not running!"
            print_info "Start with: ./mothrbox start"
            exit 1
        fi
        
        print_info "Testing MothrBox..."
        mkdir -p data
        echo "Test: $(date)" > data/test.txt
        
        print_info "Testing server..."
        if curl -s http://localhost:8000/ > /dev/null; then
            print_success "Server OK"
        else
            print_error "Server not responding"
            exit 1
        fi
        
        print_info "Testing encryption..."
        $COMPOSE_CMD run --rm mothrbox-cli aes encrypt /data/test.txt /data/test.enc "TestPass" > /dev/null 2>&1
        
        if [ -f "data/test.enc" ]; then
            print_success "Encryption OK"
        else
            print_error "Encryption failed"
            exit 1
        fi
        
        print_info "Testing decryption..."
        $COMPOSE_CMD run --rm mothrbox-cli aes decrypt /data/test.enc /data/test_dec.txt "TestPass" > /dev/null 2>&1
        
        if diff data/test.txt data/test_dec.txt > /dev/null 2>&1; then
            print_success "Decryption OK"
        else
            print_error "Decryption failed"
            exit 1
        fi
        
        rm -f data/test.txt data/test.enc data/test_dec.txt
        
        print_success "✨ All tests passed!"
        ;;
    
    encrypt)
        if [ $# -lt 2 ]; then
            print_error "Usage: ./mothrbox encrypt <file> <password>"
            exit 1
        fi
        
        FILE=$1
        PASSWORD=$2
        
        if ! $COMPOSE_CMD ps | grep -q "mothrbox-server.*Up"; then
            print_error "MothrBox not running!"
            print_info "Start with: ./mothrbox start"
            exit 1
        fi
        
        if [ ! -f "$FILE" ]; then
            print_error "File not found: $FILE"
            exit 1
        fi
        
        BASENAME=$(basename "$FILE")
        if [[ "$FILE" != data/* ]]; then
            cp "$FILE" "data/$BASENAME"
            FILE="data/$BASENAME"
        fi
        
        print_info "Encrypting and uploading..."
        $COMPOSE_CMD run --rm mothrbox-cli walrus upload-aes "/$FILE" "$PASSWORD" http://mothrbox-server:8000
        ;;
    
    decrypt)
        if [ $# -lt 3 ]; then
            print_error "Usage: ./mothrbox decrypt <blobId> <output> <password>"
            exit 1
        fi
        
        if ! $COMPOSE_CMD ps | grep -q "mothrbox-server.*Up"; then
            print_error "MothrBox not running!"
            exit 1
        fi
        
        print_info "Downloading and decrypting..."
        $COMPOSE_CMD run --rm mothrbox-cli walrus download-aes "$1" "/data/$2" "$3" http://mothrbox-server:8000
        
        if [ -f "data/$2" ]; then
            print_success "Saved to: data/$2"
        fi
        ;;
    
    keygen)
        print_info "Generating ECC keys..."
        $COMPOSE_CMD run --rm -w /data mothrbox-cli ecc keygen
        print_success "Keys: data/private.key, data/public.key"
        ;;
    
    cli)
        $COMPOSE_CMD run --rm mothrbox-cli "$@"
        ;;
    
    rebuild)
        print_info "Rebuilding..."
        $COMPOSE_CMD build --no-cache
        print_success "Done"
        ;;
    
    clean)
        print_warning "Remove all containers and data?"
        read -p "yes/no: " confirm
        if [ "$confirm" = "yes" ]; then
            $COMPOSE_CMD down -v
            docker rmi mothrbox-server mothrbox-cli 2>/dev/null || true
            print_success "Cleaned"
        fi
        ;;
    
    help|--help|-h)
        show_help
        ;;
    
    *)
        print_error "Unknown command: $COMMAND"
        show_help
        exit 1
        ;;
esac