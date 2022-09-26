use chrono::{DateTime, Local};
use serde::Serializer;
use std::time::SystemTime;

/// Serializes a [`SystemTime`] as a [`String`].
pub fn serialize<S: Serializer>(val: &SystemTime, serializer: S) -> Result<S::Ok, S::Error> {
    let datetime = DateTime::<Local>::from(val.to_owned());
    let datetime_str = datetime.format("%F %T").to_string();
    serializer.serialize_str(&datetime_str)
}

/// Serializes a [`Option<SystemTime>`] as a [`Option<String>`].
pub fn serialize_option<S: Serializer>(
    val: &Option<SystemTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match val {
        Some(s) => {
            let datetime = DateTime::<Local>::from(s.to_owned());
            let datetime_str = datetime.format("%F %T").to_string();
            serializer.serialize_some(&datetime_str)
        }
        None => serializer.serialize_none(),
    }
}
