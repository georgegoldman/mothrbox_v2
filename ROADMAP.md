# 🦋 MothrBox v2 Development Roadmap

**Project Vision:** Unified encrypted decentralized storage system combining military-grade encryption with Walrus Protocol's decentralized storage.

> **Status:** Phase 2 (Cloud Platform MVP) in progress
> **Target Date:** Q1 2026

---

## ✅ Phase 1: Core Infrastructure (COMPLETED)

### Rust Encryption Engine (`mothrbox_rs`)
- [x] AES-256-GCM encryption/decryption
- [x] ChaCha20-Poly1305 encryption/decryption
- [x] ECC P-256 (secp256r1) key generation and encryption
- [x] PBKDF2 key derivation (600,000 iterations)
- [x] File I/O operations
- [x] CLI interface for encryption operations

### TypeScript Walrus Client (`mothrbox_ts`)
- [x] Walrus Protocol SDK integration
- [x] Upload/download to Walrus network
- [x] Sui wallet integration
- [x] Blob management
- [x] JSON stdout responses for Rust communication

### System Integration
- [x] Docker containerization
- [x] Bash subprocess IPC (Rust ↔ Deno)
- [x] Unified `./mothrbox` command interface
- [x] Environment configuration (`.env`)
- [x] Volume-mounted data directory

---

## 🚀 Phase 2: Cloud Platform MVP (IN PROGRESS)
**Current Sprint | Target: Q1 2026**

### 2.1 Web Interface Foundation
- [ ] Next.js/React frontend setup
- [ ] Responsive UI design system (Tailwind/Radix UI)
- [ ] Dark mode support
- [ ] Mobile-responsive layouts

### 2.2 Sui Wallet Integration
- [ ] Sui Wallet adapter integration (`@mysten/dapp-kit`)
- [ ] Wallet connection modal
- [ ] Account balance display
- [ ] Transaction signing flow
- [ ] Network switching (testnet/mainnet)

### 2.3 NFT Key Management
- [ ] **Smart Contract:** Move contract for key NFTs
- [ ] NFT minting logic (mint metadata: algorithm, creation date)
- [ ] Key metadata storage (encrypted key wrapping)
- [ ] Transfer/ownership management
- [ ] Frontend NFT gallery (View, Copy, Export)

### 2.4 File Encryption UI
- [ ] File upload component (drag & drop)
- [ ] Algorithm selection dropdown (AES-256-GCM / ChaCha20 / ECC)
- [ ] Client-side encryption (WASM integration)
- [ ] Cost estimation before upload

### 2.5 File Management Dashboard
- [ ] Encrypted files list view
- [ ] Metadata display (name, size, algo, date)
- [ ] Download and decrypt flow
- [ ] Search, filter, and delete functionality

### 2.6 Rust WebAssembly Module
- [ ] Compile Rust encryption engine to WASM (`wasm-pack`)
- [ ] TypeScript bindings generation
- [ ] Chunked file processing for large files

### 2.7 Cost & Payments
- [ ] Walrus storage pricing API integration
- [ ] Pay-as-you-go SUI payment integration
- [ ] SUI balance check and insufficient funds warning

---

## 📦 Phase 3: Developer Tools (PLANNED)
**Timeline: Q2 2026**

### 3.1 SDK Development
**Objective:** Enable developers to integrate MothrBox into their applications.

**JavaScript/TypeScript SDK**
- [ ] NPM package setup (`@mothrbox/sdk`)
- [ ] Core encryption methods
- [ ] Wallet integration helpers
- [ ] Stream processing for large files

**Python SDK**
- [ ] PyPI package setup
- [ ] Async/await support and type hints

**Go SDK**
- [ ] Go module setup
- [ ] Concurrent upload/download support

### 3.2 SDK Features (Cross-language)
- [ ] Batch operations
- [ ] Progress callbacks
- [ ] Key caching & automatic retry logic

---

## 🐳 Phase 4: Docker & Deployment (PLANNED)
**Timeline: Q2 2026**

### 4.1 Docker Images
- [ ] Multi-stage build optimization
- [ ] Separate images (Rust service, Deno client, Web frontend)
- [ ] Docker Compose configurations (Dev/Prod/Test)

### 4.2 Cloud & Kubernetes
- [ ] Helm charts and Service definitions
- [ ] AWS/GCP/Azure deployment templates
- [ ] Self-hosting documentation

---

## 🔒 Phase 5: Security & Compliance (PLANNED)
**Timeline: Q3 2026**

- [ ] Third-party cryptography & smart contract audits
- [ ] GDPR & HIPAA compliance documentation
- [ ] 2FA & Rate limiting implementation
- [ ] HSM support & key rotation policies

---

## 📊 Phase 6: Advanced Features (PLANNED)
**Timeline: Q3-Q4 2026**

- [ ] **Enterprise:** Multi-user orgs, RBAC, Shared vaults
- [ ] **Versioning:** File version history and rollback
- [ ] **Advanced Crypto:** Threshold encryption, MPC, Post-quantum support
- [ ] **Mobile:** iOS/Android apps (React Native)

---

## 🎯 Success Metrics
- **Performance:** Encryption < 10ms/MB, API Response < 200ms
- **Adoption:** 10k+ active users, 1M+ files encrypted
- **Reliability:** 99.9% Uptime, 99.5% Upload Success Rate
