use serde::{Deserialize, Serialize};

use crate::behavior::biome::BiomeReference;

/// Controls how this biome is instantiated (and then potentially modified) during world generation of the overworld.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverworldGenerationRules {
    /// Controls the world generation climate categories that this biome can spawn for.  A single biome can be associated with multiple categories with different weightings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_for_climates: Option<Vec<GenerateForClimates>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hills_transformation: Option<Transformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutate_transformation: Option<Transformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub river_transformation: Option<Transformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shore_transformation: Option<Transformation>,
}

/// Controls the world generation climate categories that this biome can spawn for.  A single biome can be associated with multiple categories with different weightings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Transformation {
    BiomeReference(BiomeReference),
    BiomeReferenceArray(Vec<BiomeReference>),
    BiomeReferenceWeighted(Vec<WeightedBiomeReference>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeightedBiomeReference(BiomeReference, i32);

/// Name of a climate category.
/// Weight with which this biome should be selected, relative to other biomes in the same category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateForClimates(ClimateCategory, i32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClimateCategory {
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "warm")]
    Warm,
    #[serde(rename = "lukewarm")]
    Lukewarm,
    #[serde(rename = "cold")]
    Cold,
    #[serde(rename = "frozen")]
    Frozen,
}
