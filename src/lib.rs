use derive_more::Deref;
use std::{any::type_name, convert::TryFrom, fmt::Debug, ops::Deref};
use thiserror::Error;

// We require TryFrom rather than write our own blanket impl because doing the latter will conflict with std's blanket impl:
// impl<T, U> TryFrom<U> for T where U: Into<T>
pub trait ClampedInclusive:
    TryFrom<Self::Clampee, Error = OutOfBounds<Self::Clampee>> + Deref<Target = Self::Clampee>
{
    type Clampee: num::PrimInt + Debug;
    const LOWER: Self::Clampee;
    const UPPER: Self::Clampee;
    unsafe fn new_unchecked(i: Self::Clampee) -> Self;
    fn try_clamp(i: Self::Clampee) -> Result<Self, OutOfBounds<Self::Clampee>> {
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
}

#[derive(Debug, Error)]
#[error("The value {given:?} is not in the inclusive interval {lower:?}..{upper:?}")]
pub struct OutOfBounds<T: Debug> {
    lower: T,
    upper: T,
    given: T,
}

macro_rules! clamped {
    ($struct_name:ident, $clampee:ty) => {
        #[derive(Deref)]
        pub struct $struct_name<const L: $clampee, const U: $clampee>($clampee);

        impl<const L: $clampee, const U: $clampee> ClampedInclusive for $struct_name<L, U> {
            type Clampee = $clampee;
            const LOWER: $clampee = L;
            const UPPER: $clampee = U;
            unsafe fn new_unchecked(i: $clampee) -> Self {
                Self(i)
            }
        }

        impl<const L: $clampee, const U: $clampee> TryFrom<$clampee> for $struct_name<L, U> {
            type Error = OutOfBounds<$clampee>;
            fn try_from(i: $clampee) -> Result<Self, Self::Error> {
                Self::try_clamp(i)
            }
        }

        impl<const L: $clampee, const U: $clampee> Debug for $struct_name<L, U> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // #[derive(Debug)] doesn't preserve the const type parameters
                // Use type_name instead, and strip the module path
                let name = match type_name::<Self>().rsplit_once("::") {
                    Some((_pre, post)) => post,
                    None => type_name::<Self>(),
                };
                let value: &$clampee = self;
                f.debug_tuple(name).field(value).finish()
            }
        }
    };
}

clamped!(ClampedU8, u8);
clamped!(ClampedU16, u16);
clamped!(ClampedU32, u32);
clamped!(ClampedU64, u64);
clamped!(ClampedU128, u128);

clamped!(ClampedI8, i8);
clamped!(ClampedI16, i16);
clamped!(ClampedI32, i32);
clamped!(ClampedI64, i64);
clamped!(ClampedI128, i128);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_test() {
        ClampedU8::<10, 20>::try_from(10).unwrap();
        ClampedU8::<10, 20>::try_from(9).unwrap_err();
        ClampedU8::<10, 20>::try_from(21).unwrap_err();
    }
}
