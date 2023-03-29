/* Raw contents of follow_caravan.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.follow_caravan",
  "type": "object",
  "title": "Follow Caravan",
  "description": "Allows the mob to follow mobs that are in a caravan.",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "speed_multiplier": {
      "$ref": "./types/speed_multiplier.json"
    },
    "entity_types": {
      "$ref": "../types/entity_types.json",
      "description": "List of entity types that this mob can follow in a caravan.",
      "title": "Entity Types"
    },
    "entity_count": {
      "type": "integer",
      "description": "Number of entities that can be in the caravan.",
      "default": 1,
      "title": "Entity Count"
    }
  },
  "examples": [
    {
      "entity_count": 1
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowCaravan {}
