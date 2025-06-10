//! Modular reduce implementation.

use rand_core::RngCore;

use crate::u256::{MontyForm, MontyParams, U256};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Reduce 512 bits, presumably to get a non-biased field element. While
    /// taking the modulus of 512 bits produces negligible bias (method used
    /// below), there may be better algorithms with zero bias.
    pub(crate) fn reduce(bytes: &[u8; 64]) -> Self {
        // Do modulus on 512 bits using 256-bit math
        // val_512 mod M = (((2^256 mod M) * hi_256) mod M + (lo_256 mod M)) mod M

        let lo = Self::new(&U256::from_le_bytes(bytes[..32].try_into().unwrap()));
        let hi = Self::new(&U256::from_le_bytes(bytes[32..64].try_into().unwrap()));
        let hi = Self::mul(&Self::new(&MOD::TWO_TO_256_MOD_M), &hi);
        Self::add(&hi, &lo)
    }

    pub(crate) fn random(mut rng: impl RngCore) -> Self {
        let mut bytes = [0; 64];
        rng.fill_bytes(&mut bytes);
        Self::reduce(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        fields::{Field25519Params, HelioseleneParams},
        u256::MontyForm,
    };
    struct TC {
        input: &'static str,
        output: &'static str,
    }

    impl TC {
        fn input_bytes(&self) -> [u8; 64] {
            let mut input: [u8; 64] = hex::decode(self.input)
                .expect("Failed to decode hex")
                .try_into()
                .expect("Input must be 64 bytes");
            input.reverse(); // Use little endian once in binary form
            input
        }
    }

    #[test]
    fn test_reduce_field25519() {
        // Hex values are in big endian.
        const REDUCE_TESTS: [TC; 4] = [
                TC {
                    input: "70b7f6776fedc692aaa93223b6694532d97205e209f2e2cb51b49c056988041780d802b0513e6a11e7ece450e3166ce4d8a13a56cdeb3c5d731c4cac2d9bc9a1",
                    output: "3c26986aee89e3d73d0a559df6b6b2711f8e19e447f8e68b93eb7579d7cc6791",
                },
                TC {
                    input: "0a6dc2d2be742c5d0d811ee43afeef432c8d529332ad7ca541d1477b5276ede8ade6b16414b5a165ef8d94f908036056f88d5228d9f9479e247e632c9de9715f",
                    output: "3a319cac59f43735f0b82ad9c9dae44f958794025fb9c825e98eff7adb90c21b",
                },
                TC {
                    input: "f23e13f70f8369004e2b0e06772676b4f827111bc2961f80c738aca2ac6a92c638c5a561f78952fd1dff02e2078e0ea7c49e4a1a6939cf3b8304c8ee9e31f4ef",
                    output: "2dfc9c0e450ae908b86317d7b743ad849a6ad4394b827c59156e69143603c3ab",
                },
                TC {
                    input: "d32b624c8176b0d0ed780fbdc248f7df4e862e110a9bc03624ba0ebff2d9f55906d9769ab1bcde613af3e3417805b728dd025f6c0cfc87209faeb2f484cacc08",
                    output: "5f4a0df5e95b1d647ac6396c4eda824e84ed35f3a01b0f2a134ce37291253bd8",
                }
            ];

        for tc in &REDUCE_TESTS {
            type MontyFormType = MontyForm<Field25519Params>;
            let output = MontyFormType::reduce(&tc.input_bytes());
            let output = hex::encode(output.retrieve().to_be_bytes());
            assert_eq!(output, tc.output);
        }
    }

    #[test]
    fn test_reduce_helioselene_field() {
        // Hex values are in big endian.
        const REDUCE_TESTS: [TC; 4] = [
            TC {
                input: "70b7f6776fedc692aaa93223b6694532d97205e209f2e2cb51b49c056988041780d802b0513e6a11e7ece450e3166ce4d8a13a56cdeb3c5d731c4cac2d9bc9a1",
                output: "4c854e33959c8db9bf70bd7e1570b9b4c79b0cfa4371f9021422286907c70a3c",

            },
            TC {
                input: "0a6dc2d2be742c5d0d811ee43afeef432c8d529332ad7ca541d1477b5276ede8ade6b16414b5a165ef8d94f908036056f88d5228d9f9479e247e632c9de9715f",
                output: "49e818746c572bb613708a36e45a3b70e7167c1d01773d8d8e72149e0f3e16d8",

            },
            TC {
                input: "f23e13f70f8369004e2b0e06772676b4f827111bc2961f80c738aca2ac6a92c638c5a561f78952fd1dff02e2078e0ea7c49e4a1a6939cf3b8304c8ee9e31f4ef",
                output: "7e74749f65cd5ddb47d587453a0e98ecb9dcee837cbbdfd78c83ba1fd8981529",

            },
            TC {
                input: "d32b624c8176b0d0ed780fbdc248f7df4e862e110a9bc03624ba0ebff2d9f55906d9769ab1bcde613af3e3417805b728dd025f6c0cfc87209faeb2f484cacc08",
                output: "24c6031703a130382415c0cd23f7a04c597ee6fcacb2df0c00e65fbf4dd7aff6",

            }
        ];

        for tc in &REDUCE_TESTS {
            type MontyFormType = MontyForm<HelioseleneParams>;
            let output = MontyFormType::reduce(&tc.input_bytes());
            let ouput = hex::encode(output.retrieve().to_be_bytes());
            assert_eq!(ouput, tc.output);
        }
    }
}
