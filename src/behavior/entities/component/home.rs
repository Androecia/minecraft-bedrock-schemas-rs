/* Raw contents of home.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.home",
  "type": "object",
  "title": "Home",
  "description": "Saves a home pos for when the the entity is spawned.",
  "additionalProperties": false,
  "properties": {
    "restriction_radius": {
      "title": "Restriction Radius",
      "description": "The radius that the entity will be restricted to in relation to its home.",
      "type": "integer",
      "default": -1
    },
    "home_block_list": {
      "title": "Home Block List",
      "description": "Optional block list that the home position will be associated with. If any of the blocks no longer exist at that position, the home restriction is removed. Example syntax: minecraft:sand. Not supported: minecraft:sand:1",
      "type": "array",
      "items": {
        "title": "Home Block",
        "description": "Optional block that the home position will be associated with. If any of the blocks no longer exist at that position, the home restriction is removed. Example syntax: minecraft:sand. Not supported: minecraft:sand:1",
        "type": "string",
        "$ref": "../../../../general/item/identifier.json"
      }
    }
  },
  "examples": [
    {
      "restriction_radius": -1,
      "home_block_list": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Home {}
