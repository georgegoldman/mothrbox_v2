#[allow(lint(self_transfer))]
module mothrbox_move::mothrbox_move {

    public struct Ecc {}
    public struct AES {}
    public struct Chacha {}

    public struct MothrboxNFT<phantom T> has key, store {
        id: UID,
        name: std::string::String,
        description: std::string::String,
        encrypted_dek: vector<u8>,
        encryption_algo: std::string::String,
        image_url: sui::url::Url,
    }

    public fun mint<T>(
        name: std::string::String,
        encrypted_dek: vector<u8>,
        encryption_algo: std::string::String,
        ctx: &mut TxContext
    ) {
        let nft = MothrboxNFT<T> {
            id: object::new(ctx),
            name,
            encrypted_dek,
            description: b"A secure encrypted file on Walrus".to_string(),
            encryption_algo,
            image_url: sui::url::new_unsafe("https://example.com/image.png"),
        };
        transfer::transfer(nft, tx_context::sender(ctx));
    }

    public fun share_key<T>(nft: MothrboxNFT<T>, recipient: address) {
        transfer::public_transfer(nft, recipient);
    }

    public fun burn<T>(nft: MothrboxNFT<T>) {
        let MothrboxNFT { id, .. } = nft;
        object::delete(id);
    }

    // ✅ Seal policy function (expected by key server)
    public entry fun seal_approve<T>(
        _id: vector<u8>,
        _nft: &MothrboxNFT<T>
    ) {
        // If the caller can pass &_nft, they own it (or have access to it).
        // Put extra rules here later if you want (time locks, allowlists, etc).
    }
}
