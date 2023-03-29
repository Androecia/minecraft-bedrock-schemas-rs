/* Raw contents of squid_out_of_water.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.squid_out_of_water",
  "description": "Allows the squid to stick to the ground when outside water. Can only be used by the Squid.",
  "type": "object",
  "title": "Squid Out Of Water",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SquidOutOfWater {}
