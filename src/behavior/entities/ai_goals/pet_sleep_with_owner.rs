/* Raw contents of pet_sleep_with_owner.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.pet_sleep_with_owner",
  "description": "Allows the mob to be tempted by food they like.",
  "type": "object",
  "title": "Pet Sleep With Owner",
  "additionalProperties": false,
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "goal_radius": {
      "title": "Goal Radius",
      "type": "number",
      "default": 0.5,
      "description": "Distance in blocks within the mob considers it has reached the goal. This is the \"wiggle room\" to stop the AI from bouncing back and forth trying to reach a specific spot"
    },
    "search_height": {
      "title": "Search Height",
      "type": "integer",
      "default": 1,
      "description": "Height in blocks from the owner the pet can be to sleep with owner."
    },
    "search_radius": {
      "title": "Search Radius",
      "type": "integer",
      "default": 10,
      "description": "The radius that the mob will search for an owner to curl up with."
    },
    "search_range": {
      "title": "Search Range",
      "type": "integer",
      "default": 10,
      "description": "The range that the mob will search for an owner to curl up with."
    }
  },
  "examples": [
    {
      "goal_radius": 0.5,
      "search_height": 1,
      "search_range": 0,
      "search_radius": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PetSleepWithOwner {}
