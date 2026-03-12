use serde::{Deserialize, Deserializer};
use serde::de::{SeqAccess, Visitor};
use serde::de;

use std::fmt;


pub fn empty_vec_is_none<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>
{
    let s = match Option::<Vec<T>>::deserialize(deserializer) {
        Ok(v) => v,
        Err(error) => {
            return Err(error);
        }
    };

    let r = match s {
        Some(array) => {
            match array.is_empty() {
                false => Some(array),
                true => None
            }
        },
        None => None
    };
    Ok(r)
}

/// Deserialize an optional vector which can also contain optional values of type `T`.
/// Any values in the vector which are `None` are then removed. If the vector, after filtering
/// contains no items a value of `None` is returned as the final output value.
pub fn empty_vec_is_none_filter_nulls<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>
{
    let s = match Option::<Vec<Option<T>>>::deserialize(deserializer) {
        Ok(v) => v,
        Err(error) => {
            return Err(error);
        }
    };

    let r = match s {
        Some(array) => {
            // Filter the array to remove any null values.
            let result : Vec<T> = array.into_iter().filter_map(|v| v).collect();
            match result.is_empty() {
                false => Some(result),
                true => None
            }
        },
        None => None
    };

    Ok(r)
}

pub fn empty_vec_is_none_string_or_integer<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>
{
    struct ConvertedString(Option<String>);

    struct DeserializeStringOrIntegerVisitor;

    impl<'de> de::Visitor<'de> for DeserializeStringOrIntegerVisitor {
        type Value = ConvertedString;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer or a string")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(ConvertedString(Some(v.to_string())))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(ConvertedString(Some(v.to_string())))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(ConvertedString(Some(v.to_owned())))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(ConvertedString(None))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(ConvertedString(None))
        }
    }

    impl<'de> Deserialize<'de> for ConvertedString {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
        {
            deserializer.deserialize_any(DeserializeStringOrIntegerVisitor)
        }
    }

    struct SequenceVisitor;

    impl<'de> Visitor<'de> for SequenceVisitor {
        type Value = Option<Vec<String>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of numbers or strings that can be parsed as numbers")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element::<ConvertedString>()? {
                if let Some(v) = value.0 {
                    values.push(v);
                }
            }

            match values.is_empty() {
                true => Ok(None),
                false => Ok(Some(values))
            }
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_seq(SequenceVisitor)
}

#[allow(dead_code)]
pub fn optional_string_or_integer<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeStringOrU64Visitor)
}

#[allow(dead_code)]
pub fn optional_coerce_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DeserializeF64Visitor;

    impl<'de> de::Visitor<'de> for DeserializeF64Visitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer, floating point value or a string")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v as f64))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v as f64))
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.is_empty() || v == "" {
                Ok(None)
            } else {
                let v : f64 = v.trim().parse().map_err(serde::de::Error::custom)?;
                Ok(Some(v))
            }
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(DeserializeF64Visitor)
}

/// The source type can be either a vector of the type `T` or a singular instance (map) of the
/// same type `T`.
pub fn optional_vec_or_singular<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: de::DeserializeOwned
{
    struct DeserializeVecOrMapVisitor<T> {
        _holder: std::marker::PhantomData<T>
    }

    impl<T: de::DeserializeOwned> DeserializeVecOrMapVisitor<T> {
        fn new() -> Self {
            Self {
                _holder: std::marker::PhantomData{}
            }
        }
    }

    impl<'de, T: de::DeserializeOwned> de::Visitor<'de> for DeserializeVecOrMapVisitor<T> {

        type Value = Option<Vec<T>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("either a map or a sequence of maps")
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>
        {
            let map_value: T = Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(Some(vec![map_value]))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            match vec.len() {
                0 => Ok(None),
                _ => Ok(Some(vec))
            }
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error
        {
            Ok(None)
        }
    }

    let visitor : DeserializeVecOrMapVisitor<T> = DeserializeVecOrMapVisitor {
        _holder: std::marker::PhantomData{}
    };
    deserializer.deserialize_any(visitor)
}

pub(crate) fn optional_bool_or_string<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>
{
    struct DeserializeBoolOrStringVisitor;

    impl<'de> de::Visitor<'de> for DeserializeBoolOrStringVisitor {
        type Value = Option<bool>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean or a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error
        {
            if v.is_empty() || v == "" {
                Ok(None)
            } else {
                let s = v.to_lowercase();
                match s.as_str() {
                    "true" | "1" => Ok(Some(true)),
                    "false" | "0" => Ok(Some(false)),
                    _ => Err("expected 'true', 'false', '1' or '0' string for bool").map_err(serde::de::Error::custom)
                }
            }
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: de::Error
        {
            Ok(Some(v))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(DeserializeBoolOrStringVisitor)
}