/* Raw contents of random_fly.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.random_fly",
  "description": "Allows a mob to randomly fly around.",
  "type": "object",
  "title": "Random Fly",
  "additionalProperties": false,
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "avoid_damage_blocks": {
      "type": "boolean",
      "description": "If true, the mob will avoid blocks that cause damage.",
      "$comment": "UNDOCUMENTED",
      "title": "Avoid Damage Blocks"
    },
    "can_land_on_trees": {
      "type": "boolean",
      "default": true,
      "description": "If true, the mob can stop flying and land on a tree instead of the ground.",
      "title": "Can Land On Trees"
    },
    "xz_dist": {
      "type": "integer",
      "default": 10,
      "description": "Distance in blocks on ground that the mob will look for a new spot to move to. Must be at least 1",
      "title": "Xz Dist"
    },
    "y_dist": {
      "type": "integer",
      "default": 7,
      "description": "Distance in blocks that the mob will look up or down for a new spot to move to. Must be at least 1",
      "title": "Y Dist"
    },
    "y_offset": {
      "type": "integer",
      "description": "Height in blocks to add to the selected target position.",
      "$comment": "UNDOCUMENTED",
      "title": "Y Offset"
    }
  },
  "examples": [
    {
      "avoid_damage_blocks": true,
      "can_land_on_trees": true,
      "xz_dist": 10,
      "y_dist": 7,
      "y_offset": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomFly {}
