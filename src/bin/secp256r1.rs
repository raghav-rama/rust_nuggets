use num::BigUint;
use serde::{Deserialize, Serialize};
use sp1_core::utils::ec::field::{FieldParameters, MAX_NB_LIMBS};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Secp256r1Parameters;

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Secp256r1BaseField;

impl FieldParameters for Secp256r1BaseField {
    const NB_BITS_PER_LIMB: usize = 16;

    const NB_LIMBS: usize = 16;

    const NB_WITNESS_LIMBS: usize = 2 * Self::NB_LIMBS - 2;

    const MODULUS: [u8; MAX_NB_LIMBS] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFC,
    ];

    const WITNESS_OFFSET: usize = 1usize << 14;
}

fn main() {}
