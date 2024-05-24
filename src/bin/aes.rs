extern crate aes;
extern crate block_modes;
extern crate hex;
extern crate hex_literal;

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use hex_literal::hex;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    let key = hex!("000102030405060708090a0b0c0d0e0f");
    let iv = hex!("101112131415161718191a1b1c1d1e1f");

    let plaintext = b"hello world AES!";

    let mut buffer = [0u8; 32];
    let pos = plaintext.len();
    buffer[..pos].copy_from_slice(plaintext);

    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    println!("ciphertext: {:?}", hex::encode(ciphertext));

    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt(&mut buffer).unwrap();

    println!(
        "decrypted ciphertext: {:?}",
        std::str::from_utf8(decrypted_ciphertext).unwrap()
    );
}
