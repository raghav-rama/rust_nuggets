use k256::ecdsa::{
    signature::{SignerMut, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use rand::rngs::OsRng; // Recommended secure random number generator
use sha3::{Digest, Keccak256};

fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result);
    output
}

fn main() {
    // Key generation
    let mut secret_key = SigningKey::random(&mut OsRng);
    let public_key = VerifyingKey::from(&secret_key);

    // Sample message
    let message = b"This is a test message";

    // Hashing the message with keccak256
    let hash = keccak256(message);

    // Signing the hash
    let signature: Signature = secret_key.sign(&hash);

    // Verification
    assert!(public_key.verify(&hash, &signature).is_ok());

    println!("Message: {:?}", message);
    println!("Keccak256 Hash: {:?}", hash);
    println!("Signature: {:?}", signature);
}
