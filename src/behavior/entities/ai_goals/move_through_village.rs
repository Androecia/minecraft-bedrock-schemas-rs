/* Raw contents of move_through_village.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_through_village",
  "type": "object",
  "title": "Move Through Village",
  "description": "Can only be used by Villagers. Allows the villagers to create paths around the village.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "only_at_night": {
      "type": "boolean",
      "default": false,
      "description": "If true, the mob will only move through the village during night time.",
      "title": "Only At Night"
    }
  },
  "examples": [
    {
      "only_at_night": false
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveThroughVillage {}
