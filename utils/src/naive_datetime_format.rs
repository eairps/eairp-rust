use chrono::NaiveDateTime;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

// custom format for NaiveDateTime to string
const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn serialize<S>(dt: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *dt {
        Some(ref dt) => dt.format(FORMAT).to_string().serialize(serializer),
        None => "".serialize(serializer),  // for None, serialize as empty string
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_str: Option<String> = Option::deserialize(deserializer)?;
    match opt_str {
        Some(s) => NaiveDateTime::parse_from_str(&s, FORMAT)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
