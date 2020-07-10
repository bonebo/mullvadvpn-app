use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringResources {
    #[serde(rename = "string")]
    entries: Vec<StringResource>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringResource {
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}

impl IntoIterator for StringResources {
    type Item = StringResource;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
