use core::mem::size_of;
use std::error::Error;
use std::fmt::Debug;
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

pub(crate) trait IntoChunks<T> {
    fn chunks(self, size: T) -> Result<Vec<RangeInclusive<T>>, Box<dyn Error + Send + Sync>>;
}

impl<T: PrimInt + Unsigned + Step + Debug> IntoChunks<T> for RangeInclusive<T> {
    fn chunks(self, size: T) -> Result<Vec<RangeInclusive<T>>, Box<dyn Error + Send + Sync>> {
        let step = size.to_usize().ok_or(format!("Cannot convert {:?} to usize", size))?;
        let end = *self.end();
        let chunks = self
            .step_by(step)
            .map(|start| {
                if start > end - size {
                    start..=end
                } else {
                    start..=(start + size - T::one())
                }
            })
            .collect::<Vec<_>>();
        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunks_ascending() {
        assert_eq!((0u8..=10).chunks(4).unwrap(), vec![(0..=3), (4..=7), (8..=10)]);
        assert_eq!(
            (0..=u8::MAX).chunks(100).unwrap(),
            vec![(0..=99), (100..=199), (200..=255)]
        );
    }

    #[test]
    fn test_chunks_single_element() {
        assert_eq!((10u8..=10).chunks(1).unwrap(), vec![(10..=10)]);
    }

    #[test]
    #[should_panic]
    fn test_chunks_zero_step() {
        (10u8..=10).chunks(0).unwrap();
    }

    #[test]
    fn test_chunks_descending() {
        assert_eq!((10u8..=0).chunks(4).unwrap(), vec!());
    }
}
