/* Raw contents of move_towards_dwelling_restriction.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_towards_dwelling_restriction",
  "type": "object",
  "title": "Move Towards Dwelling Restriction",
  "additionalProperties": false,
  "description": "Allows mobs with the dweller component to move toward their Village area that the mob should be restricted to.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "types/speed_multiplier.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveTowardsDwellingRestriction {}
