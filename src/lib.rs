//! Bounded integers.
// names follow std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive}.

use std::{any, fmt};

/// Conversion error.
#[derive(Debug, thiserror::Error)]
#[error("The value {given:?} is not in the half-open range {lower:?}..{upper:?}")]
pub struct OutOfBounds<T: fmt::Debug> {
    lower: T,
    upper: T,
    given: T,
}

/// Conversion error.
#[derive(Debug, thiserror::Error)]
#[error("The value {given:?} is not in the inclusive range {lower:?}..")]
pub struct OutOfBoundsFrom<T: fmt::Debug> {
    lower: T,
    given: T,
}

/// Conversion error.
#[derive(Debug, thiserror::Error)]
#[error("The value {given:?} is not in the inclusive range {lower:?}..={upper:?}")]
pub struct OutOfBoundsInclusive<T: fmt::Debug> {
    lower: T,
    upper: T,
    given: T,
}

/// Conversion error.
#[derive(Debug, thiserror::Error)]
#[error("The value {given:?} is not in the exclusive range ..{upper:?}")]
pub struct OutOfBoundsTo<T: fmt::Debug> {
    upper: T,
    given: T,
}

/// Conversion error.
#[derive(Debug, thiserror::Error)]
#[error("The value {given:?} is not in the inclusive range ..={upper:?}")]
pub struct OutOfBoundsToInclusive<T: fmt::Debug> {
    upper: T,
    given: T,
}

macro_rules! clamped {
    (
        $inner:ty,
        $clamped:ident,
        $clamped_from:ident,
        $clamped_inclusive:ident,
        $clamped_to:ident,
        $clamped_to_inclusive:ident $(,)?
    ) => {
        /// An integer bound in the half-open range inclusively below and exclusively above `LOWER..UPPER`.
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $clamped<const LOWER: $inner, const UPPER: $inner>($inner);

        impl<const LOWER: $inner, const UPPER: $inner> TryFrom<$inner> for $clamped<LOWER, UPPER> {
            type Error = OutOfBounds<$inner>;
            /// # Panics
            /// In debug mode if `!(LOWER < UPPER)`
            fn try_from(inner: $inner) -> Result<Self, Self::Error> {
                debug_assert!(LOWER < UPPER);
                if inner < LOWER || inner >= UPPER {
                    Err(OutOfBounds {
                        lower: LOWER,
                        upper: UPPER,
                        given: inner,
                    })
                } else {
                    Ok(Self(inner))
                }
            }
        }

        impl<const LOWER: $inner, const UPPER: $inner> From<$clamped<LOWER, UPPER>> for $inner {
            fn from(clamped: $clamped<LOWER, UPPER>) -> $inner {
                clamped.0
            }
        }

        impl<const LOWER: $inner, const UPPER: $inner> fmt::Debug for $clamped<LOWER, UPPER> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // #[derive(Debug)] doesn't preserve the const type parameters
                // Use type_name instead, and strip the module path
                let name = match any::type_name::<Self>().rsplit_once("::") {
                    Some((_pre, post)) => post,
                    None => any::type_name::<Self>(),
                };
                f.debug_tuple(name).field(&self.0).finish()
            }
        }

        impl<const LOWER: $inner, const UPPER: $inner> PartialEq<$inner>
            for $clamped<LOWER, UPPER>
        {
            fn eq(&self, other: &$inner) -> bool {
                self.0 == *other
            }
        }

        /// An integer only bounded inclusively below `LOWER..`.
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $clamped_from<const LOWER: $inner>($inner);

        impl<const LOWER: $inner> TryFrom<$inner> for $clamped_from<LOWER> {
            type Error = OutOfBoundsFrom<$inner>;
            fn try_from(inner: $inner) -> Result<Self, Self::Error> {
                if inner < LOWER {
                    Err(OutOfBoundsFrom {
                        lower: LOWER,
                        given: inner,
                    })
                } else {
                    Ok(Self(inner))
                }
            }
        }

        impl<const LOWER: $inner> From<$clamped_from<LOWER>> for $inner {
            fn from(clamped: $clamped_from<LOWER>) -> $inner {
                clamped.0
            }
        }

        impl<const LOWER: $inner> fmt::Debug for $clamped_from<LOWER> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // #[derive(Debug)] doesn't preserve the const type parameters
                // Use type_name instead, and strip the module path
                let name = match any::type_name::<Self>().rsplit_once("::") {
                    Some((_pre, post)) => post,
                    None => any::type_name::<Self>(),
                };
                f.debug_tuple(name).field(&self.0).finish()
            }
        }

        impl<const LOWER: $inner> PartialEq<$inner> for $clamped_from<LOWER> {
            fn eq(&self, other: &$inner) -> bool {
                self.0 == *other
            }
        }

        /// An integer bound in the inclusive range below and above `LOWER..=UPPER`.
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $clamped_inclusive<const LOWER: $inner, const UPPER: $inner>($inner);

        impl<const LOWER: $inner, const UPPER: $inner> TryFrom<$inner>
            for $clamped_inclusive<LOWER, UPPER>
        {
            type Error = OutOfBoundsInclusive<$inner>;
            /// # Panics
            /// In debug mode if `!(LOWER <= UPPER)`
            fn try_from(inner: $inner) -> Result<Self, Self::Error> {
                debug_assert!(LOWER <= UPPER);
                if inner < LOWER || inner > UPPER {
                    Err(OutOfBoundsInclusive {
                        lower: LOWER,
                        upper: UPPER,
                        given: inner,
                    })
                } else {
                    Ok(Self(inner))
                }
            }
        }

        impl<const LOWER: $inner, const UPPER: $inner> From<$clamped_inclusive<LOWER, UPPER>>
            for $inner
        {
            fn from(clamped: $clamped_inclusive<LOWER, UPPER>) -> $inner {
                clamped.0
            }
        }

        impl<const LOWER: $inner, const UPPER: $inner> fmt::Debug
            for $clamped_inclusive<LOWER, UPPER>
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // #[derive(Debug)] doesn't preserve the const type parameters
                // Use type_name instead, and strip the module path
                let name = match any::type_name::<Self>().rsplit_once("::") {
                    Some((_pre, post)) => post,
                    None => any::type_name::<Self>(),
                };
                f.debug_tuple(name).field(&self.0).finish()
            }
        }

        impl<const LOWER: $inner, const UPPER: $inner> PartialEq<$inner>
            for $clamped_inclusive<LOWER, UPPER>
        {
            fn eq(&self, other: &$inner) -> bool {
                self.0 == *other
            }
        }

        /// An integer bound in the exclusive range above `..UPPER`.
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $clamped_to<const UPPER: $inner>($inner);

        impl<const UPPER: $inner> TryFrom<$inner> for $clamped_to<UPPER> {
            type Error = OutOfBoundsTo<$inner>;
            fn try_from(inner: $inner) -> Result<Self, Self::Error> {
                if inner >= UPPER {
                    Err(OutOfBoundsTo {
                        upper: UPPER,
                        given: inner,
                    })
                } else {
                    Ok(Self(inner))
                }
            }
        }

        impl<const UPPER: $inner> From<$clamped_to<UPPER>> for $inner {
            fn from(clamped: $clamped_to<UPPER>) -> $inner {
                clamped.0
            }
        }

        impl<const UPPER: $inner> fmt::Debug for $clamped_to<UPPER> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // #[derive(Debug)] doesn't preserve the const type parameters
                // Use type_name instead, and strip the module path
                let name = match any::type_name::<Self>().rsplit_once("::") {
                    Some((_pre, post)) => post,
                    None => any::type_name::<Self>(),
                };
                f.debug_tuple(name).field(&self.0).finish()
            }
        }

        impl<const UPPER: $inner> PartialEq<$inner> for $clamped_to<UPPER> {
            fn eq(&self, other: &$inner) -> bool {
                self.0 == *other
            }
        }

        /// An integer bound in the inclusive range above `..=UPPER`.
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $clamped_to_inclusive<const UPPER: $inner>($inner);

        impl<const UPPER: $inner> TryFrom<$inner> for $clamped_to_inclusive<UPPER> {
            type Error = OutOfBoundsTo<$inner>;
            fn try_from(inner: $inner) -> Result<Self, Self::Error> {
                if inner > UPPER {
                    Err(OutOfBoundsTo {
                        upper: UPPER,
                        given: inner,
                    })
                } else {
                    Ok(Self(inner))
                }
            }
        }

        impl<const UPPER: $inner> From<$clamped_to_inclusive<UPPER>> for $inner {
            fn from(clamped: $clamped_to_inclusive<UPPER>) -> $inner {
                clamped.0
            }
        }

        impl<const UPPER: $inner> fmt::Debug for $clamped_to_inclusive<UPPER> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // #[derive(Debug)] doesn't preserve the const type parameters
                // Use type_name instead, and strip the module path
                let name = match any::type_name::<Self>().rsplit_once("::") {
                    Some((_pre, post)) => post,
                    None => any::type_name::<Self>(),
                };
                f.debug_tuple(name).field(&self.0).finish()
            }
        }

        impl<const UPPER: $inner> PartialEq<$inner> for $clamped_to_inclusive<UPPER> {
            fn eq(&self, other: &$inner) -> bool {
                self.0 == *other
            }
        }
    };
}

clamped!(
    u8,
    ClampedU8,
    ClampedU8From,
    ClampedU8Inclusive,
    ClampedU8To,
    ClampedU8ToInclusive,
);
clamped!(
    u16,
    ClampedU16,
    ClampedU16From,
    ClampedU16Inclusive,
    ClampedU16To,
    ClampedU16ToInclusive,
);
clamped!(
    u32,
    ClampedU32,
    ClampedU32From,
    ClampedU32Inclusive,
    ClampedU32To,
    ClampedU32ToInclusive,
);
clamped!(
    u64,
    ClampedU64,
    ClampedU64From,
    ClampedU64Inclusive,
    ClampedU64To,
    ClampedU64ToInclusive,
);
clamped!(
    u128,
    ClampedU128,
    ClampedU128From,
    ClampedU128Inclusive,
    ClampedU128To,
    ClampedU128ToInclusive,
);
clamped!(
    usize,
    ClampedUsize,
    ClampedUsizeFrom,
    ClampedUsizeInclusive,
    ClampedUsizeTo,
    ClampedUsizeToInclusive,
);

clamped!(
    i8,
    ClampedI8,
    ClampedI8From,
    ClampedI8Inclusive,
    ClampedI8To,
    ClampedI8ToInclusive,
);
clamped!(
    i16,
    ClampedI16,
    ClampedI16From,
    ClampedI16Inclusive,
    ClampedI16To,
    ClampedI16ToInclusive,
);
clamped!(
    i32,
    ClampedI32,
    ClampedI32From,
    ClampedI32Inclusive,
    ClampedI32To,
    ClampedI32ToInclusive,
);
clamped!(
    i64,
    ClampedI64,
    ClampedI64From,
    ClampedI64Inclusive,
    ClampedI64To,
    ClampedI64ToInclusive,
);
clamped!(
    i128,
    ClampedI128,
    ClampedI128From,
    ClampedI128Inclusive,
    ClampedI128To,
    ClampedI128ToInclusive,
);
clamped!(
    isize,
    ClampedIsize,
    ClampedIsizeFrom,
    ClampedIsizeInclusive,
    ClampedIsizeTo,
    ClampedIsizeToInclusive,
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        type Clamped = ClampedU8<10, 20>;

        Clamped::try_from(9).unwrap_err();
        Clamped::try_from(10).unwrap();
        Clamped::try_from(11).unwrap();

        Clamped::try_from(19).unwrap();
        Clamped::try_from(20).unwrap_err();
        Clamped::try_from(21).unwrap_err();

        type ClampedFrom = ClampedU8From<10>;

        ClampedFrom::try_from(9).unwrap_err();
        ClampedFrom::try_from(10).unwrap();
        ClampedFrom::try_from(11).unwrap();

        type ClampedInclusive = ClampedU8Inclusive<10, 20>;

        ClampedInclusive::try_from(9).unwrap_err();
        ClampedInclusive::try_from(10).unwrap();
        ClampedInclusive::try_from(11).unwrap();

        ClampedInclusive::try_from(19).unwrap();
        ClampedInclusive::try_from(20).unwrap();
        ClampedInclusive::try_from(21).unwrap_err();

        type ClampedTo = ClampedU8To<10>;

        ClampedTo::try_from(9).unwrap();
        ClampedTo::try_from(10).unwrap_err();
        ClampedTo::try_from(11).unwrap_err();

        type ClampedToInclusive = ClampedU8ToInclusive<10>;

        ClampedToInclusive::try_from(9).unwrap();
        ClampedToInclusive::try_from(10).unwrap();
        ClampedToInclusive::try_from(11).unwrap_err();
    }
}
