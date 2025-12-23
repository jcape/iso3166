//! Serialization support for Alpha2 and Alpha3 types

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
impl Serialize for Alpha3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
impl<'de> Deserialize<'de> for Alpha3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(StrVisitor::<Self>::default())
    }
}
