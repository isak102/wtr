use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(DateTime::parse_from_rfc3339(s.as_str())
        .unwrap()
        .with_timezone(&Utc))
}
