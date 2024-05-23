use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{AeadInPlace, Aes256Gcm, Key, Nonce};
use rand::{rngs::OsRng, RngCore};

fn main() {
    let mut key = [0u8; 32]; // 256-bit key
    OsRng.fill_bytes(&mut key);
    let mut nonce = [0u8; 12]; // 96-bit nonce
    OsRng.fill_bytes(&mut nonce);
    let key = Key::from_slice(&key);
    let nonce = Nonce::from_slice(&nonce);

    let cipher = Aes256Gcm::new(key);
    let mut plaintext = b"This is a top secret message!".to_vec();
    let mut tag = [0u8; 16];
    let ciphertext = cipher
        .encrypt_in_place_detached(nonce, &[], &mut plaintext, &mut tag)
        .unwrap();

    let mut decryptedtext = ciphertext.to_vec();
    let cipher = Aes256Gcm::new(key);
    cipher
        .decrypt_in_place_detached(nonce, &[], &mut decryptedtext, &tag)
        .expect("Decryption failed!");

    println!("Plaintext: {}", String::from_utf8_lossy(&plaintext));
    println!("Ciphertext: {:?}", ciphertext);
    println!("Decryptedtext: {}", String::from_utf8_lossy(&decryptedtext));
}
