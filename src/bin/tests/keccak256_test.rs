#[cfg(test)]
mod tests {
    use k256::ecdsa::{
        signature::{SignerMut, Verifier},
        Signature, SigningKey, VerifyingKey,
    };
    use rand::rngs::OsRng;
    use sha3::{Digest, Keccak256};

    fn keccak256(input: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(input);
        let result = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&result);
        output
    }

    #[test]
    fn test_keccak256() {
        let input = b"hello world";
        let expected = [
            0x47, 0x17, 0x32, 0x85, 0xa8, 0xd7, 0x34, 0x1e, 0x5e, 0x97, 0x2f, 0xc6, 0x77, 0x28,
            0x63, 0x84, 0xf8, 0x02, 0xf8, 0xef, 0x42, 0xa5, 0xec, 0x5f, 0x03, 0xbb, 0xfa, 0x25,
            0x4c, 0xb0, 0x1f, 0xad,
        ];
        assert_eq!(keccak256(input), expected);
    }

    #[test]
    fn test_signing_and_verification() {
        let mut secret_key = SigningKey::random(&mut OsRng);
        let public_key = VerifyingKey::from(&secret_key);
        let message = b"This is a test message";
        let hash = keccak256(message);
        let signature: Signature = secret_key.sign(&hash);
        assert!(public_key.verify(&hash, &signature).is_ok());
    }

    #[test]
    fn test_different_message_verification_failure() {
        let mut secret_key = SigningKey::random(&mut OsRng);
        let public_key = VerifyingKey::from(&secret_key);
        let message = b"This is a test message";
        let another_message = b"This is another message";
        let hash = keccak256(message);
        let another_hash = keccak256(another_message);
        let signature: Signature = secret_key.sign(&hash);
        assert!(public_key.verify(&another_hash, &signature).is_err());
    }
}
