#[cfg(test)]

mod k256_message_sign_test {
    use k256::ecdsa::{
        signature::{SignerMut, Verifier},
        Signature, SigningKey, VerifyingKey,
    };
    use rand::rngs::OsRng; // Recommended secure random number generator

    #[test]
    fn it_verifies_signed_messaged() {
        // Key generation
        let mut secret_key = SigningKey::random(&mut OsRng);
        let public_key = VerifyingKey::from(&secret_key);

        // Sample message
        let message = b"This is a test message";

        // Signing
        let signature: Signature = secret_key.sign(message);

        // Verification
        assert!(public_key.verify(message, &signature).is_ok());
    }
}
