/* Raw contents of skeleton_horse_trap.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.skeleton_horse_trap",
  "description": "Allows Equine mobs to be Horse Traps and be triggered like them, spawning a lightning bolt and a bunch of horses when a player is nearby. Can only be used by Horses, Mules, Donkeys and Skeleton Horses.",
  "type": "object",
  "title": "Skeleton Horse Trap",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "duration": {
      "type": "number",
      "default": 1,
      "description": "Amount of time in seconds the trap exists. After this amount of time is elapsed, the trap is removed from the world if it hasn't been activated",
      "title": "Duration"
    },
    "within_radius": {
      "type": "number",
      "default": 0,
      "description": "Distance in blocks that the player has to be within to trigger the horse trap.",
      "title": "Within Radius"
    }
  },
  "examples": [
    {
      "duration": 1,
      "within_radius": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkeletonHorseTrap {}
