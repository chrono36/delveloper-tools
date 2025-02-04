use std::{fmt, slice::Iter};

use md5::{Digest, Md5};
use sha1::Sha1;
use sha2::Sha256;

#[warn(dead_code)]
struct Hasher {
    input: String,
    algorithm: HashAlgorithm,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HashAlgorithm {
    #[default]
    MD5,
    SHA1,
    SHA256,
}

impl HashAlgorithm {
    pub fn iter() -> Iter<'static, HashAlgorithm> {
        static DIRECTIONS: [HashAlgorithm; 3] = [
            HashAlgorithm::MD5,
            HashAlgorithm::SHA1,
            HashAlgorithm::SHA256,
        ];
        DIRECTIONS.iter()
    }

    pub fn hash(&self, input: &str) -> String {
        if input.trim().is_empty() {
            return String::new();
        }

        let text_bytes = input.as_bytes();
        match self {
            HashAlgorithm::MD5 => {
                let mut md5_hasher = Md5::new();
                md5_hasher.update(text_bytes);
                hex::encode(md5_hasher.finalize())
            }
            HashAlgorithm::SHA1 => {
                let mut sha1_hasher = Sha1::new();
                sha1_hasher.update(text_bytes);
                hex::encode(sha1_hasher.finalize())
            }
            HashAlgorithm::SHA256 => {
                let mut sha256_hasher = Sha256::new();
                sha256_hasher.update(text_bytes);
                hex::encode(sha256_hasher.finalize())
            }
        }
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Hasher {
    pub fn _hashing(&self) -> String {
        let text_bytes = self.input.as_bytes();
        match self.algorithm {
            HashAlgorithm::MD5 => {
                let mut md5_hasher = Md5::new();
                md5_hasher.update(text_bytes);
                hex::encode(md5_hasher.finalize())
            }
            HashAlgorithm::SHA1 => {
                let mut sha1_hasher = Sha1::new();
                sha1_hasher.update(text_bytes);
                hex::encode(sha1_hasher.finalize())
            }
            HashAlgorithm::SHA256 => {
                let mut sha256_hasher = Sha256::new();
                sha256_hasher.update(text_bytes);
                hex::encode(sha256_hasher.finalize())
            }
        }
    }
}
