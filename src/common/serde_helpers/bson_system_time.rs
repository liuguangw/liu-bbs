use mongodb::bson::DateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::SystemTime;

/// deserialize [`SystemTime`] from [`mongodb::bson::DateTime`].
pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime = DateTime::deserialize(deserializer)?;
    Ok(datetime.to_system_time())
}

/// Serializes a [`SystemTime`] as a [`mongodb::bson::DateTime`].
pub fn serialize<S: Serializer>(val: &SystemTime, serializer: S) -> Result<S::Ok, S::Error> {
    let datetime = DateTime::from_system_time(val.to_owned());
    datetime.serialize(serializer)
}
