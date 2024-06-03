#[cfg(test)]
mod tests {
    extern crate aes;
    extern crate block_modes;
    extern crate hex;
    extern crate hex_literal;

    use aes::Aes128;
    use block_modes::block_padding::Pkcs7;
    use block_modes::{BlockMode, Cbc};
    use hex_literal::hex;

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    fn encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
        let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
        let mut buffer = [0u8; 32];
        let pos = plaintext.len();
        buffer[..pos].copy_from_slice(plaintext);
        cipher.encrypt(&mut buffer, pos).unwrap().to_vec()
    }

    fn decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
        let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
        let mut buffer = [0u8; 32];
        buffer[..ciphertext.len()].copy_from_slice(ciphertext);
        cipher.decrypt(&mut buffer).unwrap().to_vec()
    }

    #[test]
    fn test_encrypt() {
        let key = hex!("616c616465656e6d61646166616b6161");
        let iv = hex!("696c6f766573616e6477696368657321");
        let plaintext = b"hello world AES!";

        let ciphertext = encrypt(&key, &iv, plaintext);
        let expected_ciphertext =
            hex!("DD27193726EB570DED5437020239B65285C54DC862B88EBB42B8F75C03D5F526");

        assert_eq!(
            hex::encode(&ciphertext[..16]),
            hex::encode(&expected_ciphertext[..16])
        );
    }

    #[test]
    fn test_decrypt() {
        let key = hex!("616c616465656e6d61646166616b6161");
        let iv = hex!("696c6f766573616e6477696368657321");
        let ciphertext = hex!("DD27193726EB570DED5437020239B65285C54DC862B88EBB42B8F75C03D5F526");

        let decrypted_ciphertext = decrypt(&key, &iv, &ciphertext);
        let expected_plaintext = b"hello world AES!";

        assert_eq!(
            String::from_utf8_lossy(&decrypted_ciphertext[..expected_plaintext.len()]),
            String::from_utf8_lossy(expected_plaintext)
        );
    }
}
