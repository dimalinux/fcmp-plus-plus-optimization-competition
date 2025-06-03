//! Traits provided by this crate

/// Encoding support.
pub(crate) trait Encoding: Sized {
    /// Byte array representation.
    type Repr: AsRef<[u8]> + AsMut<[u8]> + Copy + Clone + Sized;

    /// Decode from little endian bytes.
    fn from_le_bytes(bytes: Self::Repr) -> Self;

    /*    /// Encode to little endian bytes.
    fn to_le_bytes(&self) -> Self::Repr;*/

    /// Encode to bit endian bytes.
    #[cfg(test)]
    fn to_be_bytes(&self) -> Self::Repr;
}
