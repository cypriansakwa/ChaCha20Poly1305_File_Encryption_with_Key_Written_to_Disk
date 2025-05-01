use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce
};
use std::fs::{write, read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Prepare plaintext and generate random key and nonce
    let plaintext = b"Confidential message stored in a file";

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);

    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bit nonce

    // Step 2: Encrypt
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())
        .map_err(|e| format!("Encryption failed: {:?}", e))?;

    // Step 3: Save
    write("ciphertext.bin", &ciphertext)?;
    write("key.bin", &key)?;
    write("nonce.bin", &nonce)?;
    println!("Encryption done. Check ciphertext.bin");

    // Step 4: Decrypt
    let ciphertext = read("ciphertext.bin")?;
    let nonce_bytes = read("nonce.bin")?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let key_bytes = read("key.bin")?;
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    let decrypted_data = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("Decryption failed: {:?}", e))?;

    write("decrypted.txt", &decrypted_data)?;
    println!("Decryption done. Check decrypted.txt");

    Ok(())
}
