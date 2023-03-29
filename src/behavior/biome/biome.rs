use crate::general::identifier::namespace::NamespaceIdentifier;
use crate::general::version::Version;
use serde::{ser::Serializer, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// `crate::behavior::biome::tags::Vanilla` can be used here or you can use your own type that represents a string enum.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Biome {
    format_version: Version,
    #[serde(rename = "minecraft:biome")]
    minecraft_biome: BiomeDefinition,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description {
    identifier: NamespaceIdentifier<Biome>,
}

use super::tag::BiomeTag;
/// `crate::behavior::biome::tags::Vanilla` can be used here or you can use your own type that represents a string enum.

#[derive(Debug, Clone, PartialEq)]
pub struct BiomeDefinition {
    description: Description,
    components: super::components::Components,
    tags: Vec<NamespaceIdentifier<BiomeTag>>,
}

pub type BiomeReference = NamespaceIdentifier<Biome>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BiomeDefinitionRaw {
    pub description: Description,
    pub components: HashMap<NamespaceIdentifier<String>, serde_json::Value>,
}

impl<'de> Deserialize<'de> for BiomeDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let biome_def_raw = BiomeDefinitionRaw::deserialize(deserializer)?;

        let mut components = biome_def_raw.components.clone();
        let mut tags = Vec::new();

        for (key, value) in &biome_def_raw.components {
            if !key.get().starts_with("minecraft:")
                && value.is_object()
                && value.as_object().unwrap().is_empty()
            {
                components.remove(key);
                tags.push(NamespaceIdentifier::new(key.to_string()));
            }
        }

        Ok(Self {
            description: biome_def_raw.description,
            components: super::components::Components::from(components),
            tags,
        })
    }
}

// This is not very efficient...
impl From<BiomeDefinition> for BiomeDefinitionRaw {
    fn from(biome_def: BiomeDefinition) -> Self {
        // all of the tags are empty objects so we can just get the keys

        let mut components: HashMap<NamespaceIdentifier<String>, serde_json::Value> = biome_def
            .components
            .serialize(JsonSerializer)
            .unwrap()
            .as_object()
            .unwrap()
            .iter()
            .map(|(key, value)| (NamespaceIdentifier::new(key.to_string()), value.clone()))
            .collect();

        for tag in biome_def.tags {
            components.insert(NamespaceIdentifier::new(tag.get().clone()), json!({}));
        }

        Self {
            description: biome_def.description,
            components,
        }
    }
}

use serde_json::{json, value::Serializer as JsonSerializer};

impl Serialize for BiomeDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        BiomeDefinitionRaw::from(self.clone()).serialize(serializer)
    }
}
