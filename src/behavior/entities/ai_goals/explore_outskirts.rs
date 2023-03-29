/* Raw contents of explore_outskirts.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.explore_outskirts",
  "type": "object",
  "title": "Explore Outskirts",
  "description": "Allows the entity to first travel to a random point on the outskirts of the village, and then explore random points within a small distance. This goal requires \"minecraft:dweller\" and \"minecraft:navigation\" to execute.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "dist_from_boundary": {
      "title": "Dist From Boundary",
      "default": [5, 0, 5],
      "description": "The distance from the boundary the villager must be within in to explore the outskirts.",
      "$ref": "../../../../general/vectors/number3.json"
    },
    "explore_dist": {
      "title": "Explore Dist",
      "type": "number",
      "default": 5.0,
      "description": "Total distance in blocks the the entity will explore beyond the village bounds when choosing its travel point."
    },
    "max_travel_time": {
      "title": "Max Travel Time",
      "type": "number",
      "default": 60.0,
      "description": "This is the maximum amount of time an entity will attempt to reach it's travel point on the outskirts of the village before the goal exits."
    },
    "max_wait_time": {
      "title": "Max Wait Time",
      "type": "number",
      "default": 0.0,
      "description": "The wait time in seconds between choosing new explore points will be chosen on a random interval between this value and the minimum wait time. This value is also the total amount of time the entity will explore random points before the goal stops."
    },
    "min_dist_from_target": {
      "title": "Min Dist From Target",
      "type": "number",
      "default": 2.2,
      "description": "The entity must be within this distance for it to consider it has successfully reached its target."
    },
    "min_perimeter": {
      "title": "Min Perimeter",
      "type": "number",
      "default": 1.0,
      "description": "The minimum perimeter of the village required to run this goal."
    },
    "min_wait_time": {
      "title": "Min Wait Time",
      "type": "number",
      "default": 3.0,
      "description": "The wait time in seconds between choosing new explore points will be chosen on a random interval between this value and the maximum wait time."
    },
    "next_xz": {
      "title": "Next XZ",
      "type": "integer",
      "default": 5,
      "description": "A new explore point will randomly be chosen within this XZ distance of the current target position when navigation has finished and the wait timer has elapsed."
    },
    "next_y": {
      "title": "Next Y",
      "type": "integer",
      "default": 3,
      "description": "A new explore point will randomly be chosen within this Y distance of the current target position when navigation has finished and the wait timer has elapsed."
    },
    "timer_ratio": {
      "title": "Timer Ratio",
      "type": "number",
      "default": 2.0,
      "description": "Each new explore point will be chosen on a random interval between the minimum and the maximum wait time, divided by this value. This does not apply to the first explore point chosen when the goal runs."
    }
  },
  "examples": [
    {
      "explore_dist": 5,
      "wait_time": 0,
      "next_xz": 0,
      "next_y": 0,
      "min_wait_time": 0.0,
      "max_wait_time": 0.0,
      "max_travel_time": 0.0,
      "min_perimeter": 0.0,
      "min_dist_from_target": 0.0,
      "timer_ratio": 0.0,
      "dist_from_boundary": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExploreOutskirts {}
