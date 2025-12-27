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

impl Numeric {
    /// Create a new value the given string.
    ///
    /// This method will accept strings with 2-3 consecutive ASCII alphabetic characters left- or
    /// right-padded by ASCII whitespace. The strings in question don't need to be in a particular
    /// case. This is used by the [`FromStr`] implementation.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidCharset`] if the given slice does not contain ASCII characters.
    /// - [`Error::InvalidLength`] if the given slice is not 2 or 3 characters long.
    /// - [`Error::UnknownString`] if the given slice does not match any known Alpha2 code.
    pub const fn from_str_slice(s: &str) -> Result<Self, Error> {
        if !s.is_ascii() {
            return Err(Error::InvalidCharset);
        }

        let s = s.trim_ascii();

        let s_bytes = s.as_bytes();
        let s_len = s_bytes.len();
        if s_len != 2 && s_len != 3 {
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
            Self::from_alpha2(src.trim_ascii())
        } else {
            Self::from_alpha3(src)
        }
    }
}

impl Display for Numeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_u16())
    }
}

impl From<Numeric> for u16 {
    fn from(value: Numeric) -> Self {
        value.as_u16()
    }
}

impl TryFrom<u16> for Numeric {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value)
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
        Self::from_str_slice(s)
    }
}

impl From<Alpha2> for Numeric {
    fn from(value: Alpha2) -> Self {
        value.0
    }
}

impl From<Alpha3> for Numeric {
    fn from(value: Alpha3) -> Self {
        value.0
    }
}

impl PartialEq<Alpha2> for Numeric {
    fn eq(&self, other: &Alpha2) -> bool {
        self == &other.0
    }
}

impl PartialEq<Alpha3> for Numeric {
    fn eq(&self, other: &Alpha3) -> bool {
        self == &other.0
    }
}

/// A wrapper around the numeric enumeration requiring strings be Alpha-2 format.
///
/// When the `serde` feature is enabled, this type will be serialized as the Alpha-2 string.
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
    pub const fn from_alpha2(s: &str) -> Result<Self, Error> {
        match Numeric::from_alpha2(s) {
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
        match Numeric::from_u16(value) {
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
        Self::from_alpha2(value)
    }
}

impl TryFrom<&str> for Alpha2 {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_alpha2(value)
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
///
/// When the `serde` feature is enabled, this type will be serialized as the Alpha-3 string.
///
/// # Examples
///
/// ```rust
/// use iso3166_static::Alpha3;
/// const ALPHA3_USA: &str = "USA";
///
/// let alpha3 = Alpha3::from_alpha3(ALPHA3_USA).expect("valid alpha3");
///
/// assert_eq!(ALPHA3_USA, alpha3.as_str());
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Alpha3(Numeric);

impl Alpha3 {
    /// Create a new value from an Alpha2 string.
    ///
    /// This method strictly validates the given slice and only accepts valid upper-case Alpha3
    /// codes with no padding.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] if the given slice is not 3 characters.
    /// - [`Error::InvalidCharset`] if the given slice contains non-ASCII-7 characters.
    /// - [`Error::UnknownString`] if the given slice does not match any known Alpha3 code.
    pub const fn from_alpha3(s: &str) -> Result<Self, Error> {
        match Numeric::from_alpha3(s) {
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
        match Numeric::from_u16(value) {
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
        Self::from_alpha3(value)
    }
}

impl TryFrom<&str> for Alpha3 {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_alpha3(value)
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

#[cfg(test)]
mod test {
    extern crate std;

    use super::*;
    use std::string::ToString;

    const USA_EXPECTED2: &str = "US";
    const USA_EXPECTED3: &str = "USA";
    const USA_EXPECTED_U16: u16 = 840;

    #[test]
    fn numeric_display() {
        let src = Numeric::UnitedStatesOfAmerica;

        assert_eq!("840", src.to_string());
    }

    #[test]
    fn numeric_u16_roundtrip() {
        let actual = Numeric::try_from(USA_EXPECTED_U16).expect("valid u16");
        assert_eq!(USA_EXPECTED_U16, u16::from(actual));
    }

    #[yare::parameterized(
        pass = {USA_EXPECTED_U16, Ok(Numeric::UnitedStatesOfAmerica)},
        unknown = {u16::MAX, Err(Error::UnknownCode)},
        unknown2 = {123, Err(Error::UnknownCode)},
    )]
    fn numeric_from_u16(input: u16, expected: Result<Numeric, Error>) {
        let actual = Numeric::try_from(input);
        assert_eq!(expected, actual);
    }

    #[yare::parameterized(
        pass = {USA_EXPECTED2, Ok(Numeric::UnitedStatesOfAmerica)},
        trim_both = {"  US  ", Err(Error::InvalidLength)},
        unknown = {"XX", Err(Error::UnknownString)},
    )]
    fn numeric_from_alpha2(input: &str, expected: Result<Numeric, Error>) {
        let actual = Numeric::from_alpha2(input);
        assert_eq!(expected, actual);
    }

    #[yare::parameterized(
        pass = {USA_EXPECTED3, Ok(Numeric::UnitedStatesOfAmerica)},
        trim_both = {"  USA  ", Err(Error::InvalidLength)},
        unknown = {"XXX", Err(Error::UnknownString)},
    )]
    fn numeric_from_alpha3(input: &str, expected: Result<Numeric, Error>) {
        let actual = Numeric::from_alpha3(input);
        assert_eq!(expected, actual);
    }

    /// This test exercises the `FromStr` implementation of `Numeric`, which in turn exercises
    /// [`Numeric::from_str_slice`].
    #[yare::parameterized(
        pass = {USA_EXPECTED3, Ok(Numeric::UnitedStatesOfAmerica)},
        trim_both2 = {"  US   ", Ok(Numeric::UnitedStatesOfAmerica)},
        trim_start2 = {"     US", Ok(Numeric::UnitedStatesOfAmerica)},
        trim_end2 = {"US    ", Ok(Numeric::UnitedStatesOfAmerica)},
        trim_both3 = {"  USA  ", Ok(Numeric::UnitedStatesOfAmerica)},
        trim_start3 = {"      USA", Ok(Numeric::UnitedStatesOfAmerica)},
        trim_end3 = {"USA     ", Ok(Numeric::UnitedStatesOfAmerica)},
        unknown = {"XXX", Err(Error::UnknownString)},
        poop = {"💩💩💩", Err(Error::InvalidCharset)},
        length = {"XXXX", Err(Error::InvalidLength)},
    )]
    fn numeric_try_from_str(input: &str, expected: Result<Numeric, Error>) {
        let actual = Numeric::try_from(input);
        assert_eq!(expected, actual);
    }

    #[yare::parameterized(
        pass = {USA_EXPECTED2, Ok(Alpha2::from_numeric(Numeric::UnitedStatesOfAmerica))},
    )]
    fn alpha2_from_str(input: &str, expected: Result<Alpha2, Error>) {
        let actual = Alpha2::from_str(input);
        assert_eq!(expected, actual);
    }

    #[yare::parameterized(
        pass = {USA_EXPECTED3, Ok(Alpha3::from_numeric(Numeric::UnitedStatesOfAmerica))},
    )]
    fn alpha3_from_str(input: &str, expected: Result<Alpha3, Error>) {
        let actual = Alpha3::from_str(input);
        assert_eq!(expected, actual);
    }
}
