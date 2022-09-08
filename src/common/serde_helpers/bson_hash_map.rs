use mongodb::bson::Document;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

/// deserialize [`std::time::SystemTime`] from [`mongodb::bson::DateTime`].
pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let document = Document::deserialize(deserializer)?;
    let mut h_map = HashMap::new();
    for (k, v) in &document {
        h_map.insert(k.to_string(), v.as_str().unwrap().to_string());
    }
    Ok(h_map)
}
