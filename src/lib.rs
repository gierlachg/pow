#![forbid(unsafe_code)]
#![feature(type_alias_impl_trait)]
#![feature(step_trait)]

use std::convert::TryFrom;
use std::error::Error;
use std::ops::RangeInclusive;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::hasher::{Hasher, Sha256Hasher};
use crate::num::{IntoChunks, ToBytes};

mod hasher;
mod num;

/// Finds a suitable 4-byte prefix so that, a SHA256 hash of the prefix combined with the original string of bytes,
/// has two last bytes as 0xCA, 0xFE. The returned tuple consists of SHA256 string found and 4-byte prefix used.
///
/// ```
/// use pow::prove;
/// let payload = "129df964b701d0b8e72fe7224cc71643cf8e000d122e72f742747708f5e3bb6294c619604e52dcd8f5446da7e9ff7459d1d3cefbcc231dd4c02730a22af9880c";
///
/// let (hash, nonce) = prove(&payload).unwrap().unwrap();
///
/// assert_eq!(hash, "6681edd1d36af256c615bf6dcfcda03c282c3e0871bd75564458d77c529dcafe");
/// assert_eq!(nonce, "00003997");
/// ```
pub fn prove(payload: &str) -> Result<Option<(String, String)>, Box<dyn Error + Send + Sync>> {
    let payload = hex::decode(payload)?;

    let hasher = Sha256Hasher::new();
    let predicate = |hash: &[u8]| hash[hash.len() - 2..hash.len()] == [0xCA, 0xFE];

    let solution = (0..=u32::MAX)
        .chunks(usize::try_from(u32::MAX)? / rayon::current_num_threads())?
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter_map(|range| solve(range, &payload, hasher.clone(), predicate))
        .find_any(|_| true)
        .map(|(hash, nonce)| (hex::encode(hash), hex::encode(nonce)));
    Ok(solution)
}

fn solve<T, H, P>(range: RangeInclusive<T>, payload: &[u8], mut hasher: H, predicate: P) -> Option<(Vec<u8>, Vec<u8>)>
where
    T: ToBytes,
    H: Hasher,
    P: Fn(&[u8]) -> bool,
    RangeInclusive<T>: Iterator,
    <RangeInclusive<T> as Iterator>::Item: ToBytes,
{
    let mut bytes: Vec<u8> = [&vec![0u8; T::SIZE], payload].concat();
    range.into_iter().find_map(|nonce| {
        bytes.splice(..T::SIZE, nonce.to_be_bytes());
        let hash = hasher.hash(&bytes);
        if predicate(hash) {
            Some((hash.to_vec(), bytes[..T::SIZE].to_vec()))
        } else {
            None
        }
    })
}
