/* Raw contents of pushable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.pushable",
  "additionalProperties": false,
  "type": "object",
  "title": "Pushable",
  "description": "Defines what can push an entity between other entities and pistons.",
  "required": [],
  "properties": {
    "is_pushable": {
      "type": "boolean",
      "default": true,
      "description": "Whether the entity can be pushed by other entities.",
      "title": "Is Pushable"
    },
    "is_pushable_by_piston": {
      "type": "boolean",
      "default": true,
      "description": "Whether the entity can be pushed by pistons safely.",
      "title": "Is Pushable By Piston"
    }
  },
  "examples": [
    {
      "is_pushable": true,
      "is_pushable_by_piston": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pushable {}
