"use client";

import { useEffect, useState, useRef } from "react";
// Import your local WASM package
import initWasm, {
  ecc_generate_key,
  ecc_encrypt,
  ecc_decrypt,
  aes_encrypt,
  aes_decrypt,
  chacha_encrypt, // Note: Matches your Rust typo "ecrypt"
  chacha_decrypt,
} from "mothrbox-wasm";

export default function Home() {
  const [logs, setLogs] = useState<string[]>([]);
  const [isReady, setIsReady] = useState(false);
  const initialized = useRef(false);

  // Helper to print logs to screen
  const log = (msg: string) => setLogs((prev) => [...prev, msg]);

  useEffect(() => {
    if (initialized.current) return;
    initialized.current = true;

    initWasm()
      .then(() => {
        setIsReady(true);
        log("✅ WASM Module Loaded Successfully");
      })
      .catch((err) => log(`❌ Failed to load WASM: ${err}`));
  }, []);

  const runTests = () => {
    setLogs([]); // Clear logs
    log("🚀 Starting Crypto Tests...");

    try {
      const textEncoder = new TextEncoder();
      const textDecoder = new TextDecoder();
      const message = "Mothrbox Secret Message 2026";
      const data = textEncoder.encode(message);

      // --- TEST 1: ECC Identity & Encryption ---
      log("\n--- 1. Testing ECC (Identity) ---");
      const alice = ecc_generate_key();
      const bob = ecc_generate_key();

      log(`👤 Alice Keys Generated (Pub: ${alice.public_key.length} bytes)`);
      log(`👤 Bob Keys Generated (Pub: ${bob.public_key.length} bytes)`);

      log("🔒 Alice encrypting for Bob...");
      const eccCipher = ecc_encrypt(data, bob.public_key, alice.private_key);
      log(`📦 Encrypted Blob Size: ${eccCipher.length} bytes`);

      log("🔓 Bob decrypting...");
      const eccDecrypted = ecc_decrypt(eccCipher, bob.private_key);
      const eccResult = textDecoder.decode(eccDecrypted);

      if (eccResult === message) {
        log(`✅ ECC Success! Message: "${eccResult}"`);
      } else {
        log(`❌ ECC Failed. Got: ${eccResult}`);
      }

      // --- TEST 2: AES Encryption ---
      log("\n--- 2. Testing AES-256 ---");
      const password = "my-super-secret-password";

      log("🔒 Encrypting with AES...");
      const aesCipher = aes_encrypt(data, password);
      log(`📦 AES Blob Size: ${aesCipher.length} bytes`);

      log("🔓 Decrypting with AES...");
      const aesDecrypted = aes_decrypt(aesCipher, password);
      const aesResult = textDecoder.decode(aesDecrypted);

      if (aesResult === message) {
        log(`✅ AES Success! Message: "${aesResult}"`);
      } else {
        log(`❌ AES Failed.`);
      }

      // --- TEST 3: ChaCha20 Encryption ---
      log("\n--- 3. Testing ChaCha20 ---");

      log("🔒 Encrypting with ChaCha20...");
      const chachaCipher = chacha_encrypt(data, password);
      log(`📦 ChaCha Blob Size: ${chachaCipher.length} bytes`);

      log("🔓 Decrypting with ChaCha20...");
      const chachaDecrypted = chacha_decrypt(chachaCipher, password);
      const chachaResult = textDecoder.decode(chachaDecrypted);

      if (chachaResult === message) {
        log(`✅ ChaCha Success! Message: "${chachaResult}"`);
      } else {
        log(`❌ ChaCha Failed.`);
      }
    } catch (e) {
      log(`❌ CRITICAL ERROR: ${e}`);
      console.error(e);
    }
  };

  return (
    <main className="flex min-h-screen flex-col items-center bg-gray-900 p-24 font-mono text-white">
      <h1 className="mb-8 text-4xl font-bold text-blue-400">
        Mothrbox WASM Tester
      </h1>

      <div className="mb-8">
        Status:{" "}
        {isReady ? (
          <span className="text-green-400">Ready</span>
        ) : (
          <span className="text-red-400">Loading...</span>
        )}
      </div>

      <button
        onClick={runTests}
        disabled={!isReady}
        className="rounded-lg bg-blue-600 px-6 py-3 font-bold transition-all hover:bg-blue-500 disabled:opacity-50"
      >
        Run Diagnostics
      </button>

      <div className="mt-8 min-h-[400px] w-full max-w-2xl rounded-lg border border-gray-700 bg-black p-6">
        <h3 className="mb-4 border-b border-gray-800 pb-2 text-gray-400">
          Console Output:
        </h3>
        {logs.map((line, i) => (
          <div key={i} className="mb-1 break-all whitespace-pre-wrap">
            {line}
          </div>
        ))}
      </div>
    </main>
  );
}
