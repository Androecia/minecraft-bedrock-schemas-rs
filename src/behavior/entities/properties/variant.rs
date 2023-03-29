/* Raw contents of variant.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.variant",
  "additionalProperties": false,
  "type": "object",
  "title": "Variant",
  "description": "Used to differentiate the component group of a variant of an entity from others (e.g. ocelot, villager) Parameters",
  "required": ["value"],
  "properties": {
    "value": {
      "type": "integer",
      "default": 0,
      "description": "The ID of the variant. By convention, 0 is the ID of the base entity",
      "title": "Value"
    }
  },
  "examples": [{ "value": 0 }, { "value": 1 }, { "value": 2 }, { "value": 3 }, { "value": 4 }, { "value": 5 }, { "value": 6 }, { "value": 7 }, { "value": 8 }, { "value": 9 }]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {}
