use derive_more::{Deref, Display};
use std::any::type_name;
use std::{convert::TryFrom, fmt::Debug};

use crate::{Clamped, OutOfBounds};

#[derive(Display, Deref)]
pub struct ClampedUsize<const L: usize, const U: usize>(usize);

impl<const L: usize, const U: usize> Debug for ClampedUsize<L, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // #[derive(Debug)] doesn't preserve the const type parameters
        // Use type_name instead, and strip the module path
        let name = match type_name::<Self>().rsplit_once("::") {
            Some((_pre, post)) => post,
            None => type_name::<Self>(),
        };
        f.debug_tuple(name).field(&self.0).finish()
    }
}

impl<const L: usize, const U: usize> Clamped<usize> for ClampedUsize<L, U> {
    unsafe fn new_unchecked(i: usize) -> Self {
        Self(i)
    }
    const LOWER: usize = L;
    const UPPER: usize = U;
    fn unclamp(self) -> usize {
        self.0
    }
}

impl<const L: usize, const U: usize> TryFrom<usize> for ClampedUsize<L, U> {
    type Error = OutOfBounds<usize>;

    fn try_from(i: usize) -> Result<Self, Self::Error> {
        Self::try_clamp(i)
    }
}

impl<const L: usize, const U: usize> Into<usize> for ClampedUsize<L, U> {
    fn into(self) -> usize {
        self.unclamp()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn try_into_ok() {
        let c: ClampedUsize<5, 10> = 5.try_into().unwrap();
        println!("{:?}", c);
        println!("{}", c);
    }

    #[test]
    fn try_into_err() {
        let e = TryInto::<ClampedUsize<5, 10>>::try_into(4).unwrap_err();
        println!("{:?}", e);
        println!("{}", e);
    }
}
