mod capped_surface;
use std::collections::HashMap;

pub use capped_surface::*;
mod climate;
pub use climate::*;
mod consolidated_features;
pub use consolidated_features::*;
mod forced_features;
pub use forced_features::*;
mod frozen_ocean_surface;
pub use frozen_ocean_surface::*;
mod ignore_automatic_features;
pub use ignore_automatic_features::*;
mod legacy_world_generation_rules;
pub use legacy_world_generation_rules::*;
mod mesa_surface;
pub use mesa_surface::*;

mod mountain_parameters;
pub use mountain_parameters::*;
mod nether_generation_rules;
pub use nether_generation_rules::*;
mod nether_surface;
pub use nether_surface::*;
mod overworld_generation_rules;
pub use overworld_generation_rules::*;
mod overworld_height;
pub use overworld_height::*;
mod surface_material_adjustments;
pub use surface_material_adjustments::*;
mod surface_parameters;
pub use surface_parameters::*;
mod swamp_surface;
pub use swamp_surface::*;
mod the_end_surface;
pub use the_end_surface::*;

mod multinoise_generation_rule;
pub use multinoise_generation_rule::*;

use serde::{Deserialize, Serialize};

use crate::general::identifier::namespace::NamespaceIdentifier;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Components {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:climate")]
    climate: Option<Climate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:overworld_height")]
    overworld_height: Option<OverworldHeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:surface_parameters")]
    surface_parameters: Option<SurfaceParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:capped_surface")]
    capped_surface: Option<CappedSurface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:consolidated_features")]
    consolidated_features: Option<ConsolidatedFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:forced_features")]
    forced_features: Option<ForcedFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:frozen_ocean_surface")]
    frozen_ocean_surface: Option<FrozenOceanSurface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:ignore_automatic_features")]
    ignore_automatic_features: Option<IgnoreAutomaticFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:legacy_world_generation_rules")]
    legacy_world_generation_rules: Option<LegacyWorldGenerationRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:mesa_surface")]
    mesa_surface: Option<MesaSurface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:mountain_parameters")]
    mountain_parameters: Option<MountainParameters>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:multinoise_generation_rules")]
    multinoise_generation_rules: Option<MultinoiseGenerationRules>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:nether_generation_rules")]
    nether_generation_rules: Option<NetherGenerationRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:nether_surface")]
    nether_surface: Option<NetherSurface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:overworld_generation_rules")]
    overworld_generation_rules: Option<OverworldGenerationRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:surface_material_adjustments")]
    surface_material_adjustments: Option<SurfaceMaterialAdjustments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:swamp_surface")]
    swamp_surface: Option<SwampSurface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minecraft:the_end_surface")]
    the_end_surface: Option<TheEndSurface>,
}

impl From<HashMap<NamespaceIdentifier<String>, serde_json::Value>> for Components {
    fn from(map: HashMap<NamespaceIdentifier<String>, serde_json::Value>) -> Self {
        let mut components = Components {
            climate: None,
            overworld_height: None,
            surface_parameters: None,
            capped_surface: None,
            consolidated_features: None,
            forced_features: None,
            frozen_ocean_surface: None,
            ignore_automatic_features: None,
            legacy_world_generation_rules: None,
            mesa_surface: None,
            mountain_parameters: None,
            multinoise_generation_rules: None,
            nether_generation_rules: None,
            nether_surface: None,
            overworld_generation_rules: None,
            surface_material_adjustments: None,
            swamp_surface: None,
            the_end_surface: None,
        };

        for (key, value) in map {
            match key.to_string().as_str() {
                "minecraft:climate" => {
                    components.climate = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:overworld_height" => {
                    components.overworld_height = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:surface_parameters" => {
                    components.surface_parameters = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:capped_surface" => {
                    components.capped_surface = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:consolidated_features" => {
                    components.consolidated_features = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:forced_features" => {
                    components.forced_features = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:frozen_ocean_surface" => {
                    components.frozen_ocean_surface = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:ignore_automatic_features" => {
                    components.ignore_automatic_features =
                        Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:legacy_world_generation_rules" => {
                    components.legacy_world_generation_rules =
                        Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:mesa_surface" => {
                    components.mesa_surface = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:mountain_parameters" => {
                    components.mountain_parameters = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:multinoise_generation_rules" => {
                    components.multinoise_generation_rules =
                        Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:nether_generation_rules" => {
                    components.nether_generation_rules =
                        Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:nether_surface" => {
                    components.nether_surface = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:overworld_generation_rules" => {
                    components.overworld_generation_rules =
                        Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:surface_material_adjustments" => {
                    components.surface_material_adjustments =
                        Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:swamp_surface" => {
                    components.swamp_surface = Some(serde_json::from_value(value).unwrap());
                }
                "minecraft:the_end_surface" => {
                    components.the_end_surface = Some(serde_json::from_value(value).unwrap());
                }
                _ => {
                    panic!("Unknown component: {}:{:#?}", key.to_string(), value);
                }
            }
        }

        components
    }
}
