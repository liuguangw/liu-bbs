use chrono::{DateTime, Local};
use serde::Serializer;
use std::time::SystemTime;

/// Serializes a [`SystemTime`] as a [`String`].
pub fn serialize<S: Serializer>(val: &SystemTime, serializer: S) -> Result<S::Ok, S::Error> {
    let datetime = DateTime::<Local>::from(val.to_owned());
    let datetime_str = datetime.format("%F %T").to_string();
    serializer.serialize_str(&datetime_str)
}
