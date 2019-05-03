use serde::{Deserialize, Deserializer};

pub fn u64_deserializer<'de, D>(d: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    let s: String = s.chars().filter(|c| !vec![',', '%'].contains(c)).collect();
    match &s[..] {
        "" => Ok(None),
        _ => Ok(Some(s.parse::<u64>().unwrap())),
    }
}

pub fn f64_deserializer<'de, D>(d: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    let s: String = s.chars().filter(|c| !vec![',', '%'].contains(c)).collect();
    match &s[..] {
        "" => Ok(None),
        _ => Ok(Some(s.parse::<f64>().unwrap())),
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
