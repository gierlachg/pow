use sha2::digest::generic_array::GenericArray;
use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256};

pub(crate) trait Hasher: Clone {
    fn hash(&mut self, payload: &[u8]) -> &[u8];
}

#[derive(Clone)]
pub(crate) struct Sha256Hasher {
    hasher: Sha256,
    hash: GenericArray<u8, <Sha256 as Digest>::OutputSize>,
}

impl Sha256Hasher {
    pub(crate) fn new() -> Self {
        Sha256Hasher {
            hasher: Sha256::new(),
            hash: GenericArray::default(),
        }
    }
}

impl Hasher for Sha256Hasher {
    fn hash(&mut self, payload: &[u8]) -> &[u8] {
        self.hasher.update(payload);
        self.hasher.finalize_into_reset(&mut self.hash);
        self.hash.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let mut hasher = Sha256Hasher::new();

        assert_eq!(
            hasher.hash(&[1, 2, 3, 4, 5]).to_vec(),
            hasher.hash(&[1, 2, 3, 4, 5]).to_vec()
        );
    }
}
