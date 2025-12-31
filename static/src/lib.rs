//! Static ISO 3166 Data

#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "serde")]
mod serde_;

use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

iso3166_macros::generate!(lukes_json = "all.json");

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::UnknownCode => f.write_str("Unknown Code"),
            Error::UserAssigned => f.write_str("User Assigned"),
            Error::InvalidLength => f.write_str("Invalid Length"),
            Error::InvalidCharset => f.write_str("Invalid Character Set"),
        }
    }
}

impl Display for Numeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}

impl From<Numeric> for u16 {
    fn from(value: Numeric) -> Self {
        value as u16
    }
}

impl TryFrom<u16> for Numeric {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value)
    }
}

impl TryFrom<Alpha2> for Numeric {
    type Error = Error;

    fn try_from(value: Alpha2) -> Result<Self, Self::Error> {
        Self::from_alpha2(value)
    }
}

impl TryFrom<Alpha3> for Numeric {
    type Error = Error;

    fn try_from(value: Alpha3) -> Result<Self, Self::Error> {
        Self::from_alpha3(value)
    }
}

impl Display for Alpha2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl TryFrom<Numeric> for Alpha2 {
    type Error = Error;

    fn try_from(value: Numeric) -> Result<Self, Self::Error> {
        Self::from_numeric(value)
    }
}

impl TryFrom<Alpha3> for Alpha2 {
    type Error = Error;

    fn try_from(value: Alpha3) -> Result<Self, Self::Error> {
        Self::from_alpha3(value)
    }
}

impl TryFrom<&str> for Alpha2 {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str_slice(value)
    }
}

impl FromStr for Alpha2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_slice(s)
    }
}

impl Display for Alpha3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl TryFrom<Numeric> for Alpha3 {
    type Error = Error;

    fn try_from(value: Numeric) -> Result<Self, Self::Error> {
        Self::from_numeric(value)
    }
}

impl TryFrom<Alpha2> for Alpha3 {
    type Error = Error;

    fn try_from(value: Alpha2) -> Result<Self, Self::Error> {
        Self::from_alpha2(value)
    }
}

impl TryFrom<&str> for Alpha3 {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str_slice(value)
    }
}

impl FromStr for Alpha3 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_slice(s)
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
        pass = {Alpha2::UnitedStatesOfAmerica, Ok(Numeric::UnitedStatesOfAmerica)},
        user = {Alpha2::UserZZ, Err(Error::UserAssigned)},
    )]
    fn numeric_from_alpha2(input: Alpha2, expected: Result<Numeric, Error>) {
        let actual = Numeric::try_from(input);
        assert_eq!(expected, actual);
    }

    #[yare::parameterized(
        pass = {Alpha3::UnitedStatesOfAmerica, Ok(Numeric::UnitedStatesOfAmerica)},
        user = {Alpha3::UserZZZ, Err(Error::UserAssigned)},
    )]
    fn numeric_from_alpha3(input: Alpha3, expected: Result<Numeric, Error>) {
        let actual = Numeric::try_from(input);
        assert_eq!(expected, actual);
    }

    #[yare::parameterized(
        pass = {USA_EXPECTED2, Ok(Alpha2::UnitedStatesOfAmerica)},
        unknown = {"QB", Err(Error::UnknownCode)},
        length = {"USA", Err(Error::InvalidLength)},
        poop = {"💩", Err(Error::InvalidCharset)},
    )]
    fn alpha2_from_str(input: &str, expected: Result<Alpha2, Error>) {
        let actual = Alpha2::from_str(input);
        assert_eq!(expected, actual);
    }

    #[yare::parameterized(
        pass = {USA_EXPECTED2, Ok(Alpha2::UnitedStatesOfAmerica)},
        unknown = {"QB", Err(Error::UnknownCode)},
    )]
    fn alpha2_try_from(input: &str, expected: Result<Alpha2, Error>) {
        let actual = Alpha2::try_from(input);
        assert_eq!(expected, actual);
    }

    #[yare::parameterized(
        pass = {USA_EXPECTED3, Ok(Alpha3::UnitedStatesOfAmerica)},
        fail = {"BBB", Err(Error::UnknownCode)},
        length = {"USAID", Err(Error::InvalidLength)},
        poop = {"💩", Err(Error::InvalidCharset)},
    )]
    fn alpha3_from_str(input: &str, expected: Result<Alpha3, Error>) {
        let actual = Alpha3::from_str(input);
        assert_eq!(expected, actual);
    }

    #[yare::parameterized(
        pass = {USA_EXPECTED3, Ok(Alpha3::UnitedStatesOfAmerica)},
        fail = {"BBB", Err(Error::UnknownCode)},
    )]
    fn alpha3_try_from(input: &str, expected: Result<Alpha3, Error>) {
        let actual = Alpha3::try_from(input);
        assert_eq!(expected, actual);
    }
}
