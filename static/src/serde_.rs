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

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value = u16::try_from(v).map_err(E::custom)?;
        Numeric::from_u16(value).map_err(E::custom)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value = u16::try_from(v).map_err(E::custom)?;
        Numeric::from_u16(value).map_err(E::custom)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value = u16::try_from(v).map_err(E::custom)?;
        Numeric::from_u16(value).map_err(E::custom)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value = u16::try_from(v).map_err(E::custom)?;
        Numeric::from_u16(value).map_err(E::custom)
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value = u16::try_from(v).map_err(E::custom)?;
        Numeric::from_u16(value).map_err(E::custom)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value = u16::try_from(v).map_err(E::custom)?;
        Numeric::from_u16(value).map_err(E::custom)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value = u16::try_from(v).map_err(E::custom)?;
        Numeric::from_u16(value).map_err(E::custom)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Numeric::from_u16(v).map_err(E::custom)
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

#[cfg(test)]
mod test {
    use crate::{Alpha2, Alpha3, Numeric};

    const NUMERIC: Numeric = Numeric::UnitedStatesOfAmerica;
    const NUMERIC_JSON: &str = "840";

    const ALPHA2: Alpha2 = Alpha2::from_numeric(NUMERIC);
    const ALPHA2_JSON: &str = "\"US\"";

    const ALPHA3: Alpha3 = Alpha3::from_numeric(NUMERIC);
    const ALPHA3_JSON: &str = "\"USA\"";

    #[test]
    fn numeric() {
        let json = serde_json::to_string(&NUMERIC).expect("numeric serialization");
        assert_eq!(NUMERIC_JSON, json);

        let actual = serde_json::from_str::<Numeric>(&json).expect("numeric deserialization");
        assert_eq!(NUMERIC, actual);
    }

    #[test]
    fn alpha2() {
        let json = serde_json::to_string(&ALPHA2).expect("alpha2 serialization");
        assert_eq!(ALPHA2_JSON, json);

        let actual = serde_json::from_str::<Alpha2>(&json).expect("alpha2 deserialization");
        assert_eq!(ALPHA2, actual);
    }

    #[test]
    fn alpha3() {
        let json = serde_json::to_string(&ALPHA3).expect("alpha2 serialization");
        assert_eq!(ALPHA3_JSON, json);

        let actual = serde_json::from_str::<Alpha3>(&json).expect("alpha2 deserialization");
        assert_eq!(ALPHA3, actual);
    }
}
