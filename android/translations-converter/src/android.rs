use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringResources {
    #[serde(rename = "string")]
    entries: Vec<StringResource>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringResource {
    name: String,
    #[serde(rename = "$value")]
    value: String,
}
