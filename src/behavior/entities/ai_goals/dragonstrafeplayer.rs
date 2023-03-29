/* Raw contents of dragonstrafeplayer.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.dragonstrafeplayer",
  "additionalProperties": false,
  "type": "object",
  "title": "Dragonstrafeplayer",
  "description": "Allows this entity to fly around looking for a player to shoot fireballs at. Note: This behavior can only be used by the ender_dragon entity type.",
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
    "fireball_range": {
      "title": "Fireball Range",
      "type": "number",
      "default": 64,
      "description": "Maximum distance of this entity's fireball attack while strafing."
    },
    "flight_speed": {
      "title": "Flight Speed",
      "type": "number",
      "default": 0.6,
      "description": "The speed this entity moves while this behavior is not active."
    },
    "switch_direction_probability": {
      "title": "Switch Direction Probability",
      "type": "number",
      "default": 0.125,
      "description": "Percent chance to to switch this entity's strafe direction between clockwise and counterclockwise. Switch direction chance occurs each time a new target is chosen (1.0 = 100%)."
    },
    "target_in_range_and_in_view_time": {
      "title": "Target In Range And In View Time",
      "type": "number",
      "default": 0.25,
      "description": "Time (in seconds) the target must be in fireball range, and in view [ie, no solid terrain in-between the target and this entity], before a fireball can be shot."
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
    },
    "view_angle": {
      "title": "View Angle",
      "type": "number",
      "default": 10,
      "description": "The target must be within \"view_angle\" degrees of the dragon's current rotation before a fireball can be shot."
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dragonstrafeplayer {}
