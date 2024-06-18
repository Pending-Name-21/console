use serde_json;

pub fn serialize_to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("Failed to serialize")
}
