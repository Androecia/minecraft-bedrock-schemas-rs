/* Raw contents of can_fly.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.can_fly",
  "type": "object",
  "title": "Can Fly",
  "additionalProperties": false,
  "description": "Marks the entity as being able to fly, the pathfinder won't be restricted to paths where a solid block is required underneath it.",
  "required": [],
  "properties": {
    "value": {
      "type": "boolean",
      "default": true,
      "description": "Marks the entity as being able to fly, the pathfinder won't be restricted to paths where a solid block is required underneath it.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanFly {}
