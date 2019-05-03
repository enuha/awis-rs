use serde::{Deserialize, Deserializer};

pub fn u64_deserializer<'de, D>(d: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    match &s[..] {
        "" => Ok(None),
        _ => Ok(Some(s.replace(",", "").parse::<u64>().unwrap())),
    }
}

pub fn f64_deserializer<'de, D>(d: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    match &s[..] {
        "" => Ok(None),
        // TODO use a regular expression to replace % too, and use f64 rather than strings in percentages
        _ => Ok(Some(s.replace(",", "").parse::<f64>().unwrap())),
    }
}

pub fn string_deserializer<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    match &s[..] {
        "" => Ok(None),

        _ => Ok(Some(s.parse::<String>().unwrap())),
    }
}
