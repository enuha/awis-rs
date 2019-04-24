use serde::Deserializer;

fn u64_deserializer<'de, D>(d: D) -> Result<Option<u64>, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(d)?;
    match &s[..] {
        "" => Ok(None),
        _ => Ok(Some(s.parse::<u64>().unwrap())),
    }
}

fn string_deserializer<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(d)?;
    match &s[..] {
        "" => Ok(None),

        _ => Ok(Some(s.parse::<String>().unwrap())),
    }
}
