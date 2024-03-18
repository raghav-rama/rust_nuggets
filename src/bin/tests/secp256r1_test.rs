use num::{BigUint, Num};
use serde::{Deserialize, Serialize};
use sp1_core::utils::ec::{
    field::{FieldParameters, MAX_NB_LIMBS},
    weierstrass::WeierstrassParameters,
    EllipticCurveParameters,
};

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

    fn modulus() -> BigUint {
        BigUint::from_bytes_le(&Self::MODULUS)
    }
}

impl EllipticCurveParameters for Secp256r1Parameters {
    type BaseField = Secp256r1BaseField;
}

impl WeierstrassParameters for Secp256r1Parameters {
    const A: [u16; MAX_NB_LIMBS] = [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0001, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFC,
    ];

    const B: [u16; MAX_NB_LIMBS] = [
        0x5AC6, 0x35D8, 0xAA3A, 0x93E7, 0xB3EB, 0xBD55, 0x7698, 0x86BC, 0x651D, 0x06B0, 0xCC53,
        0xB0F6, 0x3BCE, 0x3C3E, 0x27D2, 0x604B, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
    ];

    fn generator() -> (BigUint, BigUint) {
        let x = BigUint::from_str_radix(
            "6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296",
            16,
        )
        .unwrap();
        let y = BigUint::from_str_radix(
            "4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5",
            16,
        )
        .unwrap();
        (x, y)
    }

    fn prime_group_order() -> BigUint {
        BigUint::from_str_radix(
            "FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551",
            16,
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::BigUint;

    #[test]
    fn test_modulus() {
        let modulus = Secp256r1BaseField::modulus();
        assert_eq!(
            modulus,
            BigUint::from_bytes_le(&Secp256r1BaseField::MODULUS)
        );
    }

    #[test]
    fn test_generator() {
        let (x, y) = Secp256r1Parameters::generator();
        assert_eq!(
            x,
            BigUint::from_str_radix(
                "6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296",
                16
            )
            .unwrap()
        );
        assert_eq!(
            y,
            BigUint::from_str_radix(
                "4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5",
                16
            )
            .unwrap()
        );
    }

    #[test]
    fn test_prime_group_order() {
        let order = Secp256r1Parameters::prime_group_order();
        assert_eq!(
            order,
            BigUint::from_str_radix(
                "FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551",
                16
            )
            .unwrap()
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use num::BigUint;

    #[test]
    fn test_curve_parameters() {
        assert_eq!(
            Secp256r1BaseField::modulus(),
            BigUint::from_bytes_le(&Secp256r1BaseField::MODULUS)
        );
        assert_eq!(
            Secp256r1Parameters::generator(),
            (
                BigUint::from_str_radix(
                    "6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296",
                    16
                )
                .unwrap(),
                BigUint::from_str_radix(
                    "4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5",
                    16
                )
                .unwrap()
            )
        );
        assert_eq!(
            Secp256r1Parameters::prime_group_order(),
            BigUint::from_str_radix(
                "FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551",
                16
            )
            .unwrap()
        );
    }
}
