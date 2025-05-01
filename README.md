# ChaCha20Poly1305 File Encryption & Decryption

This Rust example demonstrates how to securely encrypt and decrypt files using the `ChaCha20Poly1305` AEAD (Authenticated Encryption with Associated Data) cipher. The key and nonce are generated randomly and stored securely in memory, ensuring that sensitive data is protected.

---

## 📦 Dependencies

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
chacha20poly1305 = { version = "0.10", features = ["rand_core"] }
rand = "0.8"
```
## 🔧 How It Works

### Step 1: Prepare Key and Nonce  
The `ChaCha20Poly1305` key (256 bits) and nonce (96 bits) are generated using a secure random number generator (`OsRng`).

### Step 2: Encrypt the Data  
The plaintext is encrypted using the generated key and nonce. The ciphertext is stored in a binary file, alongside the key and nonce.

### Step 3: Save the Data  
The resulting ciphertext, key, and nonce are written to files:
- `ciphertext.bin` (contains the encrypted data)
- `key.bin` (contains the encryption key)
- `nonce.bin` (contains the nonce)

### Step 4: Decrypt the Data  
The program reads the ciphertext, key, and nonce from their respective files, then decrypts the ciphertext using the same key and nonce. The decrypted data is saved in a `decrypted.txt` file.

---

## 🧪 Example Output

When you run the code, you should see output like:

```text
Encryption done. Check ciphertext.bin
Decryption done. Check decrypted.txt
```
## 🔐 Security Notes
- ✅ The key and nonce **never leave memory**.
- ✅ The nonce is **unique per encryption**, as required by AEAD ciphers.
- ❌ This example **does not persist data or handle key rotation** — it's intended for secure, short-term, in-memory encryption.

### For real-world use:
- 🔑 Manage keys using a **secure key management system (KMS)**.
- 🚫 **Never reuse a nonce** with the same key.
- 🧹 Securely **zeroize sensitive data** if needed.

---

## 📚 References
- [ChaCha20Poly1305 crate documentation](https://docs.rs/chacha20poly1305)
- [AEAD encryption principles](https://en.wikipedia.org/wiki/Authenticated_encryption)
- [RFC 8439: ChaCha20 and Poly1305 for IETF Protocols](https://datatracker.ietf.org/doc/html/rfc8439)

