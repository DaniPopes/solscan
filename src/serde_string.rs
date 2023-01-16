use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S: Serializer>(value: &String, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&value)
}

pub fn deserialize<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d)
}

pub mod option {
    use super::*;

    pub fn serialize<S: Serializer>(value: &Option<String>, s: S) -> Result<S::Ok, S::Error> {
        if let Some(value) = value {
            s.serialize_some(&value)
        } else {
            s.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match <Option<String>>::deserialize(d) {
            Ok(Some(s)) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(s))
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

    pub fn serialize<S: Serializer>(value: &Vec<String>, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(value.len()))?;
        for value in value {
            seq.serialize_element(&value)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        <Vec<String>>::deserialize(d)
    }
}
