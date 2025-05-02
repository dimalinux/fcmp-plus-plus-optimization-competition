//! Wrapper type for non-zero integers.
use core::ops::Deref;

use subtle::CtOption;

use crate::bigint::Zero;

/// Wrapper type for non-zero integers.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct NonZero<T: Zero>(T);

impl<T> NonZero<T>
where
    T: Zero,
{
    /// Create a new non-zero integer.
    pub(crate) fn new(n: T) -> CtOption<Self> {
        let is_zero = n.is_zero();
        CtOption::new(Self(n), !is_zero)
    }
}

impl<T> AsRef<T> for NonZero<T>
where
    T: Zero,
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for NonZero<T>
where
    T: Zero,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
