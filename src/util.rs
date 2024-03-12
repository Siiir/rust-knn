pub use cmp::by_dist_sq::ComparedByDistSq;
pub mod cmp;

pub use ascii_char::AsciiChar7Bit;
pub mod ascii_char {
    use derive_more::Into;
    use std::{fmt::Display, str::FromStr};

    #[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Into)]
    pub struct AsciiChar7Bit(
        // Invariant: Encodes a valid 1-byte UTF-8 string on its own.
        u8,
    );

    impl AsciiChar7Bit {
        pub const COMMA: Self = Self(b',');
    }
    impl Display for AsciiChar7Bit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(
                std::str::from_utf8(&[self.0])
                    .expect("self.0 should encode a valid 1-byte UTF-8 string on its own."),
            )
        }
    }
    impl FromStr for AsciiChar7Bit {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.as_bytes() {
                &[byte] => Ok(Self(byte)),
                _ => Err(anyhow::anyhow!(
                    "Wrong number of bytes. Expected 1. Got {}",
                    s.len()
                )),
            }
        }
    }
}

use crate::IrisSpecies;
use std::collections::HashMap;

pub fn mode(classifications: impl Iterator<Item = IrisSpecies>) -> anyhow::Result<IrisSpecies> {
    let mut counts = HashMap::new();
    for classification in classifications {
        *counts.entry(classification).or_insert(0) += 1;
    }
    let mode = counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap();
    Ok(mode)
}
