/* Raw contents of is_biome.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_biome",
  "type": "object",
  "title": "Is Biome",
  "description": "Tests whether the Subject is currently in the named biome.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Tests whether the Subject is currently in the named biome."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "The Biome type to test.",
      "type": "string",
      "enum": [
        "beach",
        "desert",
        "extreme_hills",
        "flat",
        "forest",
        "ice",
        "jungle",
        "mesa",
        "mushroom_island",
        "ocean",
        "plain",
        "river",
        "savanna",
        "stone_beach",
        "swamp",
        "taiga",
        "the_end",
        "the_nether"
      ],
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "is_biome",
      "value": "beach"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsBiome {}
