//! Static ISO 3166 Data

#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "serde")]
mod serde_;

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

iso3166_macros::generate_m49!();

impl From<Numeric> for u16 {
    fn from(value: Numeric) -> Self {
        value.as_u16()
    }
}

impl TryFrom<u16> for Numeric {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from_u16(value)
    }
}

impl TryFrom<&str> for Numeric {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        <Self as FromStr>::from_str(value)
    }
}

impl FromStr for Numeric {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_ascii();

        if !s.is_ascii() {
            return Err(Error::InvalidCharset);
        }

        let s_bytes = s.as_bytes();
        let s_len = s_bytes.len();
        if s_len > 3 {
            return Err(Error::InvalidLength);
        }

        let mut src_bytes = [b' '; 3];

        src_bytes[0] = s_bytes[0];
        src_bytes[1] = s_bytes[1];
        if s_len == 3 {
            src_bytes[2] = s_bytes[2];
        }
        src_bytes.make_ascii_uppercase();

        #[allow(unsafe_code)]
        // SAFETY: this is safe becase we'e already established the source string is ASCII
        let src = unsafe { str::from_utf8_unchecked(&src_bytes) };

        if s_len == 2 {
            Self::try_from_alpha2(src.trim())
        } else {
            Self::try_from_alpha3(src)
        }
    }
}

/// A wrapper around the numeric enumeration requiring strings be Alpha-2 format.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Alpha2(Numeric);

impl Alpha2 {
    /// Create a new value from an Alpha2 string.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] if the given slice is not 2 characters.
    /// - [`Error::InvalidCharset`] if the given slice does not contain ASCII characters.
    /// - [`Error::UnknownString`] if the given slice does not match any known Alpha2 code.
    pub const fn from_str_slice(s: &str) -> Result<Self, Error> {
        match Numeric::try_from_alpha2(s) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(err),
        }
    }

    /// Create a new value from an unsigned integer.
    ///
    /// # Errors
    ///
    /// - [`Error::UnknownCode`] if the given code value is not a numeric code.
    pub const fn from_u16(value: u16) -> Result<Self, Error> {
        match Numeric::try_from_u16(value) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(err),
        }
    }

    /// Create a new value from an Alpha3 code.
    #[must_use]
    pub const fn from_alpha3(value: Alpha3) -> Self {
        Self(value.0)
    }

    /// Create a new value from a numeric code.
    #[must_use]
    pub const fn from_numeric(value: Numeric) -> Self {
        Self(value)
    }

    /// Retrieve the static Alpha2 code string for this value.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0.as_alpha2_str()
    }
}

impl Display for Alpha2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl FromStr for Alpha2 {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_str_slice(value)
    }
}

impl TryFrom<&str> for Alpha2 {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str_slice(value)
    }
}

impl TryFrom<u16> for Alpha2 {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value)
    }
}

impl From<Alpha3> for Alpha2 {
    fn from(value: Alpha3) -> Self {
        Self::from_alpha3(value)
    }
}

impl From<Numeric> for Alpha2 {
    fn from(value: Numeric) -> Self {
        Self::from_numeric(value)
    }
}

impl PartialEq<Alpha3> for Alpha2 {
    fn eq(&self, other: &Alpha3) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Numeric> for Alpha2 {
    fn eq(&self, other: &Numeric) -> bool {
        self.0 == *other
    }
}

/// A wrapper around the numeric enumeration requiring strings be Alpha-2 format.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Alpha3(Numeric);

impl Alpha3 {
    /// Create a new value from an Alpha2 string.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] if the given slice is not 3 characters.
    /// - [`Error::InvalidCharset`] if the given slice does not contain ASCII characters.
    /// - [`Error::UnknownString`] if the given slice does not match any known Alpha3 code.
    pub const fn from_str_slice(s: &str) -> Result<Self, Error> {
        match Numeric::try_from_alpha3(s) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(err),
        }
    }

    /// Create a new value from an unsigned integer.
    ///
    /// # Errors
    ///
    /// - [`Error::UnknownCode`] if the given code value is not a numeric code.
    pub const fn from_u16(value: u16) -> Result<Self, Error> {
        match Numeric::try_from_u16(value) {
            Ok(value) => Ok(Self(value)),
            Err(err) => Err(err),
        }
    }

    /// Create a new value from an Alpha2 code.
    #[must_use]
    pub const fn from_alpha2(value: Alpha2) -> Self {
        Self(value.0)
    }

    /// Create a new value from a numeric code.
    #[must_use]
    pub const fn from_numeric(value: Numeric) -> Self {
        Self(value)
    }

    /// Retrieve the static Alpha2 code string for this value.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0.as_alpha3_str()
    }
}

impl Display for Alpha3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl FromStr for Alpha3 {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_str_slice(value)
    }
}

impl TryFrom<&str> for Alpha3 {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str_slice(value)
    }
}

impl TryFrom<u16> for Alpha3 {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value)
    }
}

impl From<Alpha2> for Alpha3 {
    fn from(value: Alpha2) -> Self {
        Self::from_alpha2(value)
    }
}

impl From<Numeric> for Alpha3 {
    fn from(value: Numeric) -> Self {
        Self::from_numeric(value)
    }
}

impl PartialEq<Alpha2> for Alpha3 {
    fn eq(&self, other: &Alpha2) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Numeric> for Alpha3 {
    fn eq(&self, other: &Numeric) -> bool {
        self.0 == *other
    }
}
