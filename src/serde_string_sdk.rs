use serde::{Deserialize, Deserializer, Serializer};
use std::{fmt::Display, str::FromStr};

pub fn serialize<T: ToString, S: Serializer>(value: T, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&value.to_string())
}

pub fn deserialize<'de, E, T, D>(d: D) -> Result<T, D::Error>
where
    T: FromStr<Err = E>,
    E: Display,
    D: Deserializer<'de>,
{
    String::deserialize(d).and_then(|s| T::from_str(&s).map_err(serde::de::Error::custom))
}

pub mod option {
    use super::*;

    pub fn serialize<T: ToString, S: Serializer>(
        value: &Option<T>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(value) = value {
            s.serialize_some(&value.to_string())
        } else {
            s.serialize_none()
        }
    }

    pub fn deserialize<'de, E, T, D>(d: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr<Err = E>,
        E: Display,
        D: Deserializer<'de>,
    {
        match <Option<String>>::deserialize(d) {
            Ok(Some(s)) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    match T::from_str(&s) {
                        Ok(x) => Ok(Some(x)),
                        Err(e) => Err(serde::de::Error::custom(e)),
                    }
                }
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

pub mod vec {
    use super::*;
    use serde::ser::SerializeSeq;

    pub fn serialize<T: ToString, S: Serializer>(value: &Vec<T>, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(value.len()))?;
        for value in value {
            seq.serialize_element(&value.to_string())?;
        }
        seq.end()
    }

    pub fn deserialize<'de, E, T, D>(d: D) -> Result<Vec<T>, D::Error>
    where
        T: FromStr<Err = E>,
        E: Display,
        D: Deserializer<'de>,
    {
        <Vec<String>>::deserialize(d).and_then(|v| {
            v.into_iter()
                .map(|s| T::from_str(&s))
                .collect::<Result<Vec<T>, _>>()
                .map_err(serde::de::Error::custom)
        })
    }
}
