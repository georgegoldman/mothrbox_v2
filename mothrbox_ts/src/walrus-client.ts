import { SuiJsonRpcClient } from "npm:@mysten/sui/jsonRpc";
import { getFullnodeUrl, SuiClient } from "npm:@mysten/sui/client";
import { walrus, WalrusClient, WalrusFile } from "npm:@mysten/walrus";
import { Ed25519Keypair } from "npm:@mysten/sui/keypairs/ed25519";
import {  fromBase64 } from "npm:@mysten/bcs";

const SUI_SECRET_KEY = Deno.env.get('SUI_SECRET_KEY')

if (!SUI_SECRET_KEY) {
    throw new Error("SUI_SECRET_KEY env var is required (b64 secret key).");
}

const NETWORK = (Deno.env.get("SUI_NETWORK") ?? "testnet") as | "testnet" | "mainnet";

// const secretKeyBytes = fromBase64(SUI_SECRET_KEY);
const keypair = Ed25519Keypair.fromSecretKey(SUI_SECRET_KEY);

const rpcUrl = getFullnodeUrl(NETWORK);

export const suiClient = new SuiClient({
    url: rpcUrl
})

export const walrusClient = new WalrusClient({
    network: "testnet",
    suiClient
})

export async function uploadToWalrus(
    contents: Uint8Array,
    identifier: string,
) {
    const file = WalrusFile.from({
        contents,
        identifier
    });

    const [result] = await walrusClient.writeFiles({
        files: [file],
        epochs: 3,
        deletable: true,
        signer: keypair
    });

    return result;
}

export async function readFromWalus(blobId: string): Promise<Uint8Array> {
    const data: Uint8Array = await walrusClient.readBlob({blobId});
    return data
}