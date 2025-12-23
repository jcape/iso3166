//! Serialization support for our types

#[cfg(feature = "serde")]
use crate::Numeric;
use crate::{Alpha2, Alpha3};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
    str::FromStr,
};
use serde::{
    Deserialize, Serialize,
    de::{Error, Visitor},
};

#[cfg(feature = "serde")]
impl Serialize for Alpha2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Alpha2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StrVisitor::<Self>::default())
    }
}

#[cfg(feature = "serde")]
impl Serialize for Alpha3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Alpha3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StrVisitor::<Self>::default())
    }
}

#[cfg(feature = "serde")]
impl Serialize for Numeric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.as_u16())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Numeric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u16(NumericVisitor)
    }
}

struct NumericVisitor;

impl Visitor<'_> for NumericVisitor {
    type Value = Numeric;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("An ISO3166 numeric code")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Numeric::try_from_u16(v).map_err(E::custom)
    }
}

struct StrVisitor<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for StrVisitor<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for StrVisitor<T>
where
    T: FromStr,
    T::Err: Display,
{
    type Value = T;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("An ISO Alpha2 string code")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Self::Value::from_str(v).map_err(|err| E::custom(err))
    }
}
