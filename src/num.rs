use core::mem::size_of;
use std::convert::TryFrom;
use std::error::Error;
use std::iter::Step;
use std::ops::RangeInclusive;

use num_traits::{PrimInt, Unsigned};

pub(crate) trait ToBytes {
    const SIZE: usize;
    type T: IntoIterator<Item = u8>;

    //noinspection RsSelfConvention
    fn to_be_bytes(self) -> Self::T;
}

macro_rules! to_bytes {
    ( $( $type:ident ),* ) => {
        $(
            impl ToBytes for $type {
                const SIZE: usize = size_of::<$type>();
                type T = [u8; Self::SIZE];

                fn to_be_bytes(self) -> Self::T {
                    self.to_be_bytes()
                }
            }
        )*
    };
}
to_bytes!(u8, u16, u32, u64, u128);

pub(crate) trait IntoChunks {
    type C;
    type I: Iterator<Item = Self::C>;

    fn chunks(self, size: usize) -> Result<Self::I, Box<dyn Error + Send + Sync>>;
}

impl<T: PrimInt + Unsigned + TryFrom<usize> + Step> IntoChunks for RangeInclusive<T> {
    type C = RangeInclusive<T>;
    type I = impl Iterator<Item = Self::C>;

    fn chunks(self, step: usize) -> Result<Self::I, Box<dyn Error + Send + Sync>> {
        let size = T::try_from(step).map_err(|_| format!("Cannot convert step {:?} to usize", step))?;
        let end = *self.end();
        let chunks = self.step_by(step).map(move |start| {
            if start > end - size {
                start..=end
            } else {
                start..=(start + size - T::one())
            }
        });
        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunks_ascending() {
        assert_eq!(
            (0u8..=10).chunks(4).unwrap().collect::<Vec<_>>(),
            vec![(0..=3), (4..=7), (8..=10)]
        );
        assert_eq!(
            (0..=u8::MAX).chunks(100).unwrap().collect::<Vec<_>>(),
            vec![(0..=99), (100..=199), (200..=255)]
        );
    }

    #[test]
    fn test_chunks_single_element() {
        assert_eq!((10u8..=10).chunks(1).unwrap().collect::<Vec<_>>(), vec![(10..=10)]);
    }

    #[test]
    #[should_panic]
    fn test_chunks_zero_step() {
        (10u8..=10).chunks(0).unwrap().next();
    }

    #[test]
    fn test_chunks_descending() {
        assert_eq!((10u8..=0).chunks(4).unwrap().collect::<Vec<_>>(), vec!());
    }
}
