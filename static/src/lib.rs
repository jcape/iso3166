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
