/* Raw contents of harvest_farm_block.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.harvest_farm_block",
  "type": "object",
  "title": "Harvest Farm Block",
  "description": "Allows the entity to search within an area for farmland with air above it. If found, the entity will replace the air block by planting a seed item from its inventory on the farmland block. This goal requires \"minecraft:inventory\" and \"minecraft:navigation\" to execute. This goal will not execute if the entity does not have an item in its inventory.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "max_seconds_before_search": {
      "title": "Maximum Seconds Before Search",
      "type": "number",
      "default": 1.0,
      "description": "The maximum amount of time in seconds that the goal can take before searching for the first harvest block. The time is chosen between 0 and this number."
    },
    "search_cooldown_max_seconds": {
      "title": "Search Cooldown Maximum Seconds",
      "type": "number",
      "default": 8.0,
      "description": "The maximum amount of time in seconds that the goal can take before searching again, after failing to find a a harvest block already. The time is chosen between 0 and this number."
    },
    "search_count": {
      "title": "Search Count",
      "type": "integer",
      "default": 0,
      "description": "The number of randomly selected blocks each tick that the entity will check within its search range and height for a valid block to move to. A value of 0 will have the mob check every block within range in one tick."
    },
    "search_height": {
      "title": "Search Height",
      "type": "integer",
      "default": 1,
      "description": "The height in blocks the entity will search within to find a valid target position."
    },
    "search_range": {
      "title": "Search Range",
      "type": "integer",
      "default": 16,
      "description": "The distance in blocks the entity will search within to find a valid target position."
    },
    "seconds_until_new_task": {
      "title": "Seconds Until New Task",
      "type": "number",
      "default": 0.5,
      "description": "The amount of time in seconds that the goal will cooldown after a successful reap/sow, before it can start again."
    }
  },
  "examples": [
    {
      "max_seconds_before_search": 0.0,
      "search_cooldown_max_seconds": 0.0,
      "seconds_until_new_task": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarvestFarmBlock {}
