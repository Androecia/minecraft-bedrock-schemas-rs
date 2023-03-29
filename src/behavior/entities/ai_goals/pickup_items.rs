/* Raw contents of pickup_items.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.pickup_items",
  "description": "Allows the mob to pick up items on the ground.",
  "additionalProperties": false,
  "type": "object",
  "title": "Pickup Items",
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "can_pickup_any_item": {
      "type": "boolean",
      "default": false,
      "description": "If true, the mob can pickup any item.",
      "title": "Can Pickup Any Item"
    },
    "can_pickup_to_hand_or_equipment": {
      "type": "boolean",
      "default": true,
      "description": "If true, the mob can pickup items to its hand or armor slots.",
      "title": "Can Pickup To Hand Or Equipment"
    },
    "cooldown_after_being_attacked": {
      "type": "number",
      "default": 20.0,
      "description": "Amount of time an offended entity needs before being willing to pick up items.",
      "title": "Cooldown After Being Attacked"
    },
    "excluded_items": {
      "title": "Excluded Items",
      "description": "List of items this mob will not pick up.",
      "type": "array",
      "items": {
        "title": "Excluded Item",
        "$ref": "../../../../general/item/descriptor.json"
      }
    },
    "goal_radius": {
      "type": "number",
      "default": 0.5,
      "description": "Distance in blocks within the mob considers it has reached the goal. This is the `wiggle room` to stop the AI from bouncing back and forth trying to reach a specific spot.",
      "title": "Goal Radius"
    },
    "max_dist": {
      "type": "number",
      "default": 0.0,
      "description": "Maximum distance this mob will look for items to pick up.",
      "title": "Maximum Dist"
    },
    "search_height": {
      "title": "Search Height",
      "type": "number",
      "default": 0,
      "description": "Height in blocks the mob will look for items to pick up.",
      "$comment": "UNDOCUMENTED"
    },
    "pickup_based_on_chance": {
      "type": "boolean",
      "default": false,
      "description": "If true, depending on the difficulty, there is a random chance that the mob may not be able to pickup items.",
      "title": "Pickup Based On Chance"
    },
    "pickup_same_items_as_in_hand": {
      "type": "boolean",
      "default": false,
      "description": "If true, the mob will pickup the same item as the item in its hand.",
      "title": "Pickup Same Items As In Hand",
      "$comment": "UNDOCUMENTED"
    },
    "track_target": {
      "type": "boolean",
      "default": false,
      "description": "If true, this mob will chase after the target as long as it's a valid target.",
      "title": "Track Target"
    }
  },
  "examples": [
    {
      "can_pickup_any_item": false,
      "can_pickup_to_hand_or_equipment": true,
      "cooldown_after_being_attacked": true,
      "excluded_items": [],
      "goal_radius": 0.5,
      "max_dist": 0,
      "pickup_based_on_chance": false,
      "track_target": false
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PickupItems {}
