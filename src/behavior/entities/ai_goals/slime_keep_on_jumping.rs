/* Raw contents of slime_keep_on_jumping.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.slime_keep_on_jumping",
  "additionalProperties": false,
  "description": "Can only be used by Slimes and Magma Cubes. Allows the mob to continuously jump around like a slime.",
  "type": "object",
  "title": "Slime Keep On Jumping",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "types/speed_multiplier.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlimeKeepOnJumping {}
