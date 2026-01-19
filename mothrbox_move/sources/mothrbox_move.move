#[allow(lint(self_transfer))]
module mothrbox_move::mothrbox_move;

// 1. The NFT Struct
// This holds the "Encrypted DEK" (Data Encryption Key) as metadata.
// The actual 1GB file is on Walrus; this NFT just holds the "key" to it.

public struct MothrboxNFT has key, store {
    id: UID,
    name: std::string::String,
    description: std::string::String,
    encrypted_dek: vector<u8>, // The AES key, encrypted by Seal
    encryption_algo: std::string::String,
    image_url: sui::url::Url,
}

// 2. Minting Function
public fun mint(
        name: std::string::String,
        encrypted_dek: vector<u8>,
        encryption_algo: std::string::String,
        ctx: &mut TxContext
) {
    let nft = MothrboxNFT {
        id: object::new(ctx),
        name,
        encrypted_dek,
        description: b"A secure encrypted file on Walrus".to_string(),
        encryption_algo,
        image_url: sui::url::new_unsafe("https://example.com/image.png"),
    };
    transfer::transfer(nft, tx_context::sender(ctx));
}

// This Logic REMAINS EXACTLY THE SAME
// Seal still just checks: "Does this user own this Object?"
public fun seal_access(
    _scope: vector<u8>,
    _nft: &MothrboxNFT,
    _ctx: &TxContext
) {
    // Ownership verification happens automatically by the Move VM
    // because the user must pass the &MothrboxNFT reference.
}
