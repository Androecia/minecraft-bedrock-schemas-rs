/* Raw contents of push_through.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.push_through",
  "type": "object",
  "title": "Push Through",
  "additionalProperties": false,
  "description": "Sets the distance through which the entity can push through.",
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "default": 0.0,
      "description": "The value of the entity's push-through, in blocks.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "value": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushThrough {}
