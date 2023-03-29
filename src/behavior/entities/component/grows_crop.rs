/* Raw contents of grows_crop.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.grows_crop",
  "type": "object",
  "title": "Grows Crop",
  "description": "Could increase crop growth when entity walks over crop.",
  "additionalProperties": false,
  "properties": {
    "chance": {
      "type": "number",
      "default": 0,
      "description": "Value between 0-1. Chance of success per tick.",
      "minimum": 0,
      "maximum": 1,
      "title": "Chance"
    },
    "charges": {
      "type": "integer",
      "default": 10,
      "description": "Number of charges.",
      "title": "Charges"
    }
  },
  "examples": [
    {
      "chance": 0,
      "charges": 10
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrowsCrop {}
