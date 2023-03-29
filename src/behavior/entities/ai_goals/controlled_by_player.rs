/* Raw contents of controlled_by_player.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.controlled_by_player",
  "additionalProperties": false,
  "type": "object",
  "title": "Controlled By Player",
  "description": "Allows the entity to be controlled by the player using an item in the item_controllable property (required). Also requires the minecraft:movement property, and the minecraft:rideable property. On every tick, the entity will attempt to rotate towards where the player is facing with the control item whilst simultaneously moving forward.",
  "properties": {
    "priority": { "$ref": "./types/priority.json" },
    "fractional_rotation": {
      "title": "Fractional Rotation",
      "type": "number",
      "default": 0.5,
      "description": "The entity will attempt to rotate to face where the player is facing each tick. The entity will target this percentage of their difference in their current facing angles each tick (from 0.0 to 1.0 where 1.0 = 100%). This is limited by FractionalRotationLimit. A value of 0.0 will result in the entity no longer turning to where the player is facing."
    },
    "fractional_rotation_limit": {
      "title": "Fractional Rotation Limit",
      "type": "number",
      "default": 5.0,
      "description": "Limits the total degrees the entity can rotate to face where the player is facing on each tick."
    },
    "mount_speed_multiplier": {
      "title": "Mount Speed Multiplier",
      "type": "number",
      "default": 1.0,
      "description": "Speed multiplier of mount when controlled by player."
    }
  },
  "examples": [
    {
      "mount_speed_multiplier": 1
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlledByPlayer {}
