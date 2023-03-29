/* Raw contents of skin_id.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.skin_id",
  "type": "object",
  "title": "Skin Id",
  "additionalProperties": false,
  "description": "Skin ID value. Can be used to differentiate skins, such as base skins for villagers.",
  "required": [],
  "properties": {
    "value": {
      "type": "integer",
      "default": 0,
      "description": "The ID of the skin. By convention, 0 is the ID of the base skin",
      "title": "Value"
    }
  },
  "examples": [
    {
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkinId {}
