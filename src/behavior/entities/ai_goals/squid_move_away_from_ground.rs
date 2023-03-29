/* Raw contents of squid_move_away_from_ground.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.squid_move_away_from_ground",
  "description": "Allows the squid to move away from ground blocks and back to water. Can only be used by the Squid.",
  "type": "object",
  "title": "Squid Move Away From Ground",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SquidMoveAwayFromGround {}
