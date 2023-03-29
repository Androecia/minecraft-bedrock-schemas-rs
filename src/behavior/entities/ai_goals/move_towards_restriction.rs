/* Raw contents of move_towards_restriction.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_towards_restriction",
  "type": "object",
  "title": "Move Towards Restriction",
  "additionalProperties": false,
  "description": "Allows Guardians, Iron Golems and Villagers to move within their pre-defined area that the mob should be restricted to. Other mobs don't have a restriction defined.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "control_flags": {
      "type": "array",
      "items": {
        "type": "string",
        "enum": ["move", "look"],
        "description": "UNDOCUMENTED: control flags.",
        "title": "Control Flags"
      },
      "description": "UNDOCUMENTED: control flags.",
      "title": "Control Flags"
    }
  },
  "examples": [
    {
      "control_flags": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveTowardsRestriction {}
