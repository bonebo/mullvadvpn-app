use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

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

impl StringResources {
    pub fn new() -> Self {
        StringResources {
            entries: Vec::new(),
        }
    }
}

impl Deref for StringResources {
    type Target = Vec<StringResource>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for StringResources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}

impl IntoIterator for StringResources {
    type Item = StringResource;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
