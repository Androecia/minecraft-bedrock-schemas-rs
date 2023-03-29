/* Raw contents of ocelot_sit_on_block.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.ocelot_sit_on_block",
  "additionalProperties": false,
  "description": "Allows an entity to sit in place, similar to the ocelot entity animation pose.",
  "type": "object",
  "title": "Ocelot Sit On Block",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OcelotSitOnBlock {}
