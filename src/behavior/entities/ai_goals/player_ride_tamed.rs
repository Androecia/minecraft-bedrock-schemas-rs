/* Raw contents of player_ride_tamed.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.player_ride_tamed",
  "description": "Allows the mob to be ridden by the player after being tamed.",
  "type": "object",
  "title": "Player Ride Tamed",
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
pub struct PlayerRideTamed {}
