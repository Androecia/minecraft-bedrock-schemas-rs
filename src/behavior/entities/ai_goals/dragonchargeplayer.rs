/* Raw contents of dragonchargeplayer.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.dragonchargeplayer",
  "additionalProperties": false,
  "type": "object",
  "title": "Dragonchargeplayer",
  "description": "Allows this entity to attack a player by charging at them. The player is chosen by the \"minecraft:behavior.dragonscanning\". Note: This behavior can only be used by the ender_dragon entity type.",
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "active_speed": {
      "title": "Active Speed",
      "type": "number",
      "default": 1,
      "description": "The speed this entity moves when this behavior has started or while it's active."
    },
    "continue_charge_threshold_time": {
      "title": "Continue Charge Threshold Time",
      "type": "number",
      "default": 0.5,
      "description": "If the dragon is outside the \"target_zone\" for longer than \"continue_charge_threshold_time\" seconds, the charge is canceled."
    },
    "flight_speed": {
      "title": "Flight Speed",
      "type": "number",
      "default": 0.6,
      "description": "The speed this entity moves while this behavior is not active."
    },
    "target_zone": {
      "title": "Target Zone",
      "$ref": "../types/range_number_type.json",
      "default": [10, 150],
      "description": "Minimum and maximum distance, from the target, this entity can use this behavior."
    },
    "turn_speed": {
      "title": "Turn Speed",
      "type": "number",
      "default": 0.7,
      "description": "The speed at which this entity turns while using this behavior."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dragonchargeplayer {}
