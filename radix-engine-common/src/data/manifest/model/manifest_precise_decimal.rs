use sbor::rust::convert::TryFrom;
#[cfg(not(feature = "alloc"))]
use sbor::rust::fmt;
use sbor::rust::vec::Vec;
use sbor::*;
use utils::copy_u8_array;

use crate::data::manifest::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManifestPreciseDecimal(pub [u8; 64]);

//========
// error
//========

/// Represents an error when parsing ManifestPreciseDecimal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseManifestPreciseDecimalError {
    InvalidLength,
}

#[cfg(not(feature = "alloc"))]
impl std::error::Error for ParseManifestPreciseDecimalError {}

#[cfg(not(feature = "alloc"))]
impl fmt::Display for ParseManifestPreciseDecimalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//========
// binary
//========

impl TryFrom<&[u8]> for ManifestPreciseDecimal {
    type Error = ParseManifestPreciseDecimalError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() != 64 {
            return Err(Self::Error::InvalidLength);
        }
        Ok(Self(copy_u8_array(slice)))
    }
}

impl ManifestPreciseDecimal {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

manifest_type!(
    ManifestPreciseDecimal,
    ManifestCustomValueKind::PreciseDecimal,
    64
);
