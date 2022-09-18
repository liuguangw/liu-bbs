use mongodb::bson::DateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::SystemTime;

/// deserialize [`Option<SystemTime>`] from [`Option<mongodb::bson::DateTime>`].
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime = Option::<DateTime>::deserialize(deserializer)?;
    let t = datetime.map(|d| d.to_system_time());
    Ok(t)
}

/// Serializes a [`Option<SystemTime>`] as a [`Option<mongodb::bson::DateTime>`].
pub fn serialize<S: Serializer>(
    val: &Option<SystemTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let datetime = val
        .as_ref()
        .map(|t| DateTime::from_system_time(t.to_owned()));
    datetime.serialize(serializer)
}
