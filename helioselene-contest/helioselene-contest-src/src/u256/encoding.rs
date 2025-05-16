//! Const-friendly decoding operations for [`Uint`]

use crate::u256::{primitives::WORD_BYTES, Encoding, U256};

impl Encoding for U256 {
    type Repr = [u8; 32];

    #[inline]
    fn from_le_bytes(bytes: Self::Repr) -> Self {
        Self::from_le_slice(&bytes)
    }

    #[inline]
    fn to_le_bytes(&self) -> Self::Repr {
        let mut result = [0u8; 32];
        self.write_le_bytes(&mut result);
        result
    }

    #[cfg(test)]
    fn to_be_bytes(&self) -> Self::Repr {
        let mut result = [0u8; 32];
        self.write_be_bytes(&mut result);
        result
    }
}

impl U256 {
    /// Create a new [`Uint`] from the provided big endian hex string.
    pub(crate) const fn from_be_hex(hex: &str) -> Self {
        let bytes = hex.as_bytes();

        assert!(
            bytes.len() == Self::BYTES * 2,
            "hex string is not the expected size"
        );

        let mut res = [0; Self::LIMBS];
        let mut buf = [0u8; WORD_BYTES];
        let mut i = 0;
        let mut err = 0;

        while i < Self::LIMBS {
            let mut j = 0;
            while j < WORD_BYTES {
                let offset = (i * WORD_BYTES + j) * 2;
                let (result, byte_err) = decode_hex_byte([bytes[offset], bytes[offset + 1]]);
                err |= byte_err;
                buf[j] = result;
                j += 1;
            }
            res[Self::LIMBS - i - 1] = u64::from_be_bytes(buf);
            i += 1;
        }

        assert!(err == 0, "invalid hex byte");

        Self::new(res)
    }

    /// Create a new [`Uint`] from the provided little endian bytes.
    pub(crate) const fn from_le_slice(bytes: &[u8]) -> Self {
        debug_assert!(bytes.len() == 32, "bytes are not the expected size");
        Self {
            // cant use slices below, because try_into() is not const
            limbs: [
                u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]),
                u64::from_le_bytes([
                    bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                    bytes[15],
                ]),
                u64::from_le_bytes([
                    bytes[16], bytes[17], bytes[18], bytes[19], bytes[20], bytes[21], bytes[22],
                    bytes[23],
                ]),
                u64::from_le_bytes([
                    bytes[24], bytes[25], bytes[26], bytes[27], bytes[28], bytes[29], bytes[30],
                    bytes[31],
                ]),
            ],
        }
    }

    /// Serialize this [`Uint`] as big-endian, writing it into the provided
    /// byte slice.
    #[cfg(test)]
    pub(crate) fn write_be_bytes(&self, out: &mut [u8]) {
        debug_assert!(out.len() == 32);
        out[0..8].copy_from_slice(&self.limbs[3].to_be_bytes());
        out[8..16].copy_from_slice(&self.limbs[2].to_be_bytes());
        out[16..24].copy_from_slice(&self.limbs[1].to_be_bytes());
        out[24..32].copy_from_slice(&self.limbs[0].to_be_bytes());
    }

    /// Serialize this [`Uint`] as little-endian, writing it into the provided
    /// byte slice.
    #[inline]
    pub(crate) fn write_le_bytes(&self, out: &mut [u8]) {
        debug_assert!(out.len() == 32);
        out[0..8].copy_from_slice(&self.limbs[0].to_le_bytes());
        out[8..16].copy_from_slice(&self.limbs[1].to_le_bytes());
        out[16..24].copy_from_slice(&self.limbs[2].to_le_bytes());
        out[24..32].copy_from_slice(&self.limbs[3].to_le_bytes());
    }
}

/// Decode a single nibble of upper or lower hex
#[inline(always)]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
const fn decode_nibble(src: u8) -> u16 {
    let byte = src as i16;
    let mut ret: i16 = -1;

    // 0-9  0x30-0x39
    // if (byte > 0x2f && byte < 0x3a) ret += byte - 0x30 + 1; // -47
    ret += (((0x2fi16 - byte) & (byte - 0x3a)) >> 8) & (byte - 47);
    // A-F  0x41-0x46
    // if (byte > 0x40 && byte < 0x47) ret += byte - 0x41 + 10 + 1; // -54
    ret += (((0x40i16 - byte) & (byte - 0x47)) >> 8) & (byte - 54);
    // a-f  0x61-0x66
    // if (byte > 0x60 && byte < 0x67) ret += byte - 0x61 + 10 + 1; // -86
    ret += (((0x60i16 - byte) & (byte - 0x67)) >> 8) & (byte - 86);

    ret as u16
}

/// Decode a single byte encoded as two hexadecimal characters.
/// Second element of the tuple is non-zero if the `bytes` values are not in the valid range
/// (0-9, a-z, A-Z).
#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
const fn decode_hex_byte(bytes: [u8; 2]) -> (u8, u16) {
    let hi = decode_nibble(bytes[0]);
    let lo = decode_nibble(bytes[1]);
    let byte = (hi << 4) | lo;
    let err = byte >> 8;
    let result = byte as u8;
    (result, err)
}
