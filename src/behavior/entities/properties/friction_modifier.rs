/* Raw contents of friction_modifier.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.friction_modifier",
  "type": "object",
  "title": "Friction Modifier",
  "additionalProperties": false,
  "description": "Defines how much does friction affect this entity.",
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "default": 1.0,
      "description": "The higher the number, the more the friction affects this entity. A value of 1.0 means regular friction, while 2.0 means twice as much",
      "title": "Value"
    }
  },
  "examples": [
    {
      "value": 1.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrictionModifier {}
