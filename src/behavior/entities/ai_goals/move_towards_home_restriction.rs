/* Raw contents of move_towards_home_restriction.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.move_towards_home_restriction",
  "type": "object",
  "title": "Move Towards Home Restriction",
  "description": "Allows mobs with the home component to move toward their pre-defined area that the mob should be restricted to.",
  "additionalProperties": false,
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
pub struct MoveTowardsHomeRestriction {}
