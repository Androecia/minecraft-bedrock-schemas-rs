use crate::general::identifier::namespace::NamespaceIdentifier;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item;

pub type ItemIdentifier = NamespaceIdentifier<Item>;
#[derive(Debug, Clone, PartialEq)]
/// Stands for item:auxValue
pub struct ItemIdentifierWithData {
    pub item: ItemIdentifier,
    pub data: i32,
}

impl ItemIdentifierWithData {
    pub fn new(item: ItemIdentifier, data: i32) -> Self {
        Self { item, data }
    }
}

impl Serialize for ItemIdentifierWithData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}:{}", self.item, self.data))
    }
}

impl<'de> Deserialize<'de> for ItemIdentifierWithData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut split = s.split(':');
        let item = split.next().unwrap();
        let data = split.next().unwrap_or("0");
        Ok(Self {
            item: item.parse().unwrap(),
            data: data.parse().unwrap(),
        })
    }
}









