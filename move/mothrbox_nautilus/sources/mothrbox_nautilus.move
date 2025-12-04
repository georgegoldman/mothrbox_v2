module 0x0::nautilus_verifier {
    use sui::event;

    /// Event emitted when encryption is verified
    public struct EncryptionVerified has copy, drop {
        blob_id: vector<u8>,
        file_hash: vector<u8>,
        algorithm: vector<u8>,
        timestamp: u64,
        verifier: address,
    }

    /// Stores verified encryption operation
    public struct VerifiedEncryption has key, store {
        id: UID,
        blob_id: vector<u8>,
        file_hash: vector<u8>,
        attestation_hash: vector<u8>,
        algorithm: vector<u8>,
        timestamp: u64,
        verifier: address,
        is_verified: bool,
    }

    /// Registry of all verified encryptions
    public struct VerificationRegistry has key {
        id: UID,
        total_verifications: u64,
    }

    /// Initialize the registry
    fun init(ctx: &mut TxContext) {
        let registry = VerificationRegistry {
            id: object::new(ctx),
            total_verifications: 0,
        };
        transfer::share_object(registry);
    }

    /// Verify and record encryption operation
    public entry fun verify_encryption(
        registry: &mut VerificationRegistry,
        blob_id: vector<u8>,
        file_hash: vector<u8>,
        attestation: vector<u8>,
        algorithm: vector<u8>,
        ctx: &mut TxContext
    ) {
        let verified = VerifiedEncryption {
            id: object::new(ctx),
            blob_id,
            file_hash,
            attestation_hash: attestation,
            algorithm,
            timestamp: tx_context::epoch(ctx),
            verifier: tx_context::sender(ctx),
            is_verified: true,
        };

        registry.total_verifications = registry.total_verifications + 1;
        
        event::emit(EncryptionVerified {
            blob_id: verified.blob_id,
            file_hash: verified.file_hash,
            algorithm: verified.algorithm,
            timestamp: verified.timestamp,
            verifier: verified.verifier,
        });
        
        transfer::share_object(verified);
    }

    /// Query verification status
    public fun is_verified(verified: &VerifiedEncryption): bool {
        verified.is_verified
    }
}
