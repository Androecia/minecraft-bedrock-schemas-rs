/*
{
    "$schema": "http://json-schema.org/draft-07/schema",
    "$id": "blockception.minecraft.resource.fog",
    "examples": [],
    "type": "object",
    "additionalProperties": false,
    "title": "Fog",
    "description": "UNDOCUMENTED.",
    "$comment": "UNDOCUMENTED",
    "definitions": {
        "colorHexOrArray": {
            "examples": ["#056bd1"],
            "oneOf": [
                {
                    "type": "array",
                    "items": [
                        { "type": "number", "minimum": 0, "maximum": 1, "title": "Red" },
                        { "type": "number", "minimum": 0, "maximum": 1, "title": "Green" },
                        { "type": "number", "minimum": 0, "maximum": 1, "title": "Blue" }
                    ]
                },
                { "type": "string", "format": "color-hex", "pattern": "^\\#[0-9a-fA-F]{6}$" }
            ]
        },

        "volumeDensityObject": {
            "type": "object",
            "additionalProperties": false,
            "required": ["max_density"],
            "examples": [{ "max_density": 0.25 }, { "max_density": 0.25, "max_density_height": 128, "zero_density_height": 20, "uniform": true }],
            "properties": {
                "max_density": {
                    "title": "Maximum Density",
                    "description": "The maximum amount of opaqueness that the ground fog will take on. A value from [0.0, 1.0].",
                    "minimum": 0,
                    "maximum": 1,
                    "type": "number"
                },
                "max_density_height": {
                    "title": "Maximum Density Height",
                    "description": "The height in blocks that the ground fog will become it's maximum density.",
                    "minimum": 0,
                    "maximum": 128,
                    "type": "number"
                },
                "zero_density_height": {
                    "title": "Zero Density Height",
                    "description": "The height in blocks that the ground fog will be completely transparent and begin to appear. This value needs to be at least 1 higher than `max_density_height`.",
                    "minimum": 0,
                    "maximum": 128,
                    "type": "number"
                },
                "uniform": { "title": "Uniform", "description": "When set to true, the density will be uniform across all heights.", "type": "boolean" }
            }
        },
        "volumeMediaObject": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "absorption": {
                    "title": "Absorption",
                    "description": "Proportion of light that is absorbed (lost) per block.",
                    "$ref": "#/definitions/colorHexOrArray"
                },
                "scattering": {
                    "title": "Scattering",
                    "description": "Proportion of light that is scattered per block.",
                    "$ref": "#/definitions/colorHexOrArray"
                }
            }
        }
    },
    "properties": {
        "format_version": { "$ref": "../../general/format_version.json" },
        "minecraft:fog_settings": {
            "title": "Fog Settings",
            "description": "The definition of a single fog.",
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "description": {
                    "title": "Description",
                    "description": "The identifying description of this fog settings.",
                    "type": "object",
                    "additionalProperties": false,
                    "properties": {
                        "identifier": {
                            "type": "string",
                            "title": "Identifier",
                            "description": "The identifier for these fog settings. The identifier must include a namespace.",
                            "$ref": "../../general/fog/identifier.json"
                        }
                    }
                },
                "distance": {
                    "title": "Distance",
                    "description": "The distance fog settings for different camera locations.",
                    "type": "object",
                    "additionalProperties": false,
                    "properties": {
                        "air": { "title": "Air", "description": "The fog settings when the camera is in the air.", "$ref": "#/definitions/defaultFogSettings" },
                        "weather": {
                            "title": "Weather",
                            "description": " The fog settings for when the camera is in the air with active weather (rain, snow, etc..).",
                            "$ref": "#/definitions/defaultFogSettings"
                        },
                        "water": { "title": "Water", "description": "The fog settings when the camera is in water.", "$ref": "#/definitions/defaultFogSettings" },
                        "lava": { "title": "Lava", "description": "The fog settings when the camera is in lava.", "$ref": "#/definitions/defaultFogSettings" },
                        "lava_resistance": {
                            "title": "Lava Resistance",
                            "description": "The fog settings when the camera is in lava and the player has the lava resistance effect active.",
                            "$ref": "#/definitions/defaultFogSettings"
                        },
                        "powder_snow": {
                            "title": "Powder Snow",
                            "description": "The fog settings when the camera is inside a Powder Snow block.",
                            "$ref": "#/definitions/defaultFogSettings"
                        }
                    }
                },
                "volumetric": {
                    "title": "Volumetric",
                    "description": "The volumetric fog settings.",
                    "type": "object",
                    "additionalProperties": false,
                    "properties": {
                        "density": {
                            "title": "Density",
                            "description": "The density settings for different camera locations.",
                            "type": "object",
                            "additionalProperties": false,
                            "properties": {
                                "air": {
                                    "title": "Air",
                                    "description": "Fog density values as light passes through air blocks.",
                                    "$ref": "#/definitions/volumeDensityObject"
                                },
                                "water": {
                                    "title": "Water",
                                    "description": "Fog density values as light passes through water blocks.",
                                    "$ref": "#/definitions/volumeDensityObject"
                                },
                                "lava": {
                                    "title": "Lava",
                                    "description": "Fog density values as light passes through lava blocks.",
                                    "$ref": "#/definitions/volumeDensityObject"
                                },
                                "lava_resistance": {
                                    "title": "Lava Resistance",
                                    "description": "Fog density values as light passes through lava blocks while the player has lava resistance.",
                                    "$ref": "#/definitions/volumeDensityObject"
                                }
                            }
                        },
                        "media_coefficients": {
                            "title": "Media Coefficients",
                            "description": "The coefficient settings for the volumetric fog in different blocks.",
                            "additionalProperties": false,
                            "type": "object",
                            "properties": {
                                "air": {
                                    "title": "Air",
                                    "description": "Fog coefficient values while light passes through air.",
                                    "$ref": "#/definitions/volumeMediaObject"
                                },
                                "water": {
                                    "title": "Water",
                                    "description": "Fog coefficient values while light passes through water.",
                                    "$ref": "#/definitions/volumeMediaObject"
                                },
                                "cloud": {
                                    "title": "Cloud",
                                    "description": "Fog coefficient values while light passes through clouds.",
                                    "$ref": "#/definitions/volumeMediaObject"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
 */


use std::fmt::{LowerHex, self};

use serde::{Deserialize, Serialize, Serializer, Deserializer, de::{Visitor, SeqAccess}};

use crate::general::{version::Version, identifier::Identifier};
#[derive(Deserialize, Serialize)]
pub struct Fog {

    #[serde( tag= )]
    format_version: Version,

    #[serde(rename = "minecraft:fog_settings")]
    fog_settings: FogSettings,

}

///
#[derive(Deserialize, Serialize)]
///
pub struct FogSettings {

    description: Description,

    distance: Distance,

    volumetric: Option<Volumetric>,

}
#[derive(Deserialize, Serialize)]
/// The identifying description of this fog settings.
pub struct Description {

    identifier: Identifier,

}


/// The volumetric fog settings.
#[derive(Deserialize, Serialize)]
pub struct Volumetric {

    /// The density settings for different camera locations.
    density: Density,

    /// The coefficient settings for the volumetric fog in different blocks.
    media_coefficients: Option<MediaCoefficients>,

}


/// The coefficient settings for the volumetric fog in different blocks.
#[derive(Deserialize, Serialize)]
pub struct MediaCoefficients {

    /// Fog coefficient values while light passes through air.
    air: Option<VolumeMediaObject>,

    /// Fog coefficient values while light passes through water.
    water: Option<VolumeMediaObject>,

    /// Fog coefficient values while light passes through clouds.
    cloud: Option<VolumeMediaObject>,

}

#[derive(Deserialize, Serialize)]
pub struct VolumeMediaObject {

        /// Proportion of light that is absorbed (lost) per block.
        absorption: Color,
        /// Proportion of light that is scattered per block.
        scattering: Color,
}





/// The density settings for different camera locations.
#[derive(Deserialize, Serialize)]
pub struct Density {

        /// Fog density values as light passes through air blocks.
        air: Option<VolumeDensity>,

        /// Fog density values as light passes through water blocks.
        water: Option<VolumeDensity>,

        /// Fog density values as light passes through lava blocks.
        lava: Option<VolumeDensity>,

        /// Fog density values as light passes through lava blocks while the player has lava resistance.
        lava_resistance: Option<VolumeDensity>,

}





#[derive(Deserialize, Serialize)]
pub struct VolumeDensity {

    /// The maximum amount of opaqueness that the ground fog will take on. A value from [0.0, 1.0].
    max_density: f32,
    /// The height in blocks that the ground fog will become it's maximum density. Max of 320.
    max_density_height: Option<i16>,
    /// The height in blocks that the ground fog will be completely transparent and begin to appear. This value needs to be at least 1 higher than `max_density_height`. Max of 320.
    zero_density_height: Option<i16>,
}




/// The distance fog settings for different camera locations.
#[derive(Deserialize, Serialize, )]
pub struct Distance {

    /// The fog settings when the camera is in the air.
    air: Option<DefaultFogSettings>,
    /// The fog settings for when the camera is in the air with active weather (rain, snow, etc..).
    weather: Option<DefaultFogSettings>,
    /// The fog settings when the camera is in water.
    water: Option<DefaultFogSettings>,
    /// The fog settings when the camera is in lava.
    lava: Option<DefaultFogSettings>,
    /// The fog settings when the camera is in lava and the player has the lava resistance effect active.
    lava_resistance: Option<DefaultFogSettings>,
    /// The fog settings when the camera is inside a Powder Snow block.
    powder_snow: Option<DefaultFogSettings>,
}

#[derive(Deserialize, Serialize)]
pub struct DefaultFogSettings {
    /// The distance from the player that the fog will begin to appear. 'fog_start' must be less than or equal to 'fog_end'.
    fog_start: f32,
    /// The distance from the player that the fog will become fully opaque. 'fog_end' must be greater than or equal to 'fog_start'.
    fog_end: f32,
    /// The color that the fog will take on.
    fog_color: Color,
    /// Determines how distance value is used. Fixed distance is measured in blocks. Dynamic distance is multiplied by the current render distance.
    render_distance_type: RenderDistanceType,
    /// Additional fog data which will slowly transition to the distance fog of current biome.
    transition_fog: Option<TransitionFog>,
}

#[derive(Deserialize, Serialize)]
pub struct TransitionFog {

    /// Initial fog that will slowly transition into water distance fog of the biome when player goes into water.
    init_fog: InitialFog,

    /// The minimum progress of fog transition.
    min_percent: f32,

    /// The time takes to reach certain progress('mid_percent') of fog transition.
    mid_seconds: u32,

    /// The progress of fog transition after 'mid_seconds' seconds.
    mid_percent: f32,

    /// Total amount of time takes to complete fog transition.
    max_seconds: u32,

}

/// Initial fog that will slowly transition into water distance fog of the biome when player goes into water.
#[derive(Deserialize, Serialize)]
pub struct InitialFog {
    /// The color that the fog will take on.
    fog_color: Color,
    /// The distance from the player that the fog will begin to appear. 'fog_start' must be less than or equal to 'fog_end'.
    fog_start: f32,
    /// The distance from the player that the fog will become fully opaque. 'fog_end' must be greater than or equal to 'fog_start'.
    fog_end: f32,
    /// Determines how distance value is used. Fixed distance is measured in blocks. Dynamic distance is multiplied by the current render distance.
    render_distance_type: RenderDistanceType,
}

impl InitialFog {
   pub fn new( fog_color: Color, fog_start: f32, fog_end: f32, render_distance_type: RenderDistanceType) -> Self {
        Self {
            fog_color,
            fog_start,
            fog_end,
            render_distance_type,
        }
    }
}







#[derive( Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Hex(String),
    Array(f32,f32,f32),
}




use std::io::Read;

use walkdir::WalkDir;
/// tests to make sure the json is parsed correctly
#[test]
fn test_json() {

    // load all of the fog json files from ./bedrock-samples/resource_pack/fogs
    let mut files = Vec::new();

    for entry in WalkDir::new("./bedrock-samples/resource_pack/fogs")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }

    // parse the json files

    for file_name in files {
        let file = std::fs::File::open(file_name.clone()).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let _fog: Fog = match serde_json::from_reader(reader) {

            Ok(fog) => fog,
            Err(e) => panic!("error parsing json: {}, \n File: {}", e,file_name),
        };
    }



}





#[derive(Deserialize, Serialize)]
pub enum RenderDistanceType {
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "render")]
    Render,
}
