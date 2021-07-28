use std::{convert::TryFrom, fmt::Debug, ops::Deref};
mod impls;
use thiserror::Error;

// We require TryFrom rather than write our own blanket impl because doing the latter will conflict with std's blanket impl
// impl<T, U> TryFrom<U> for T where U: Into<T>
pub trait Clamped<T: Ord + Debug + Default + Copy>:
    TryFrom<T, Error = OutOfBounds<T>> + Into<T> + Deref<Target = T>
{
    unsafe fn new_unchecked(i: T) -> Self;
    const LOWER: T;
    const UPPER: T;
    fn try_clamp(i: T) -> Result<Self, OutOfBounds<T>> {
        if i < Self::LOWER || i > Self::UPPER {
            Err(OutOfBounds {
                lower: Self::LOWER,
                upper: Self::UPPER,
                given: i,
            })
        } else {
            Ok(unsafe { Self::new_unchecked(i) })
        }
    }
    fn unclamp(self) -> T;
}

#[derive(Debug, Error)]
#[error("The value {given:?} is not in the inclusive interval {lower:?}..{upper:?}")]
pub struct OutOfBounds<T: Debug> {
    lower: T,
    upper: T,
    given: T,
}
