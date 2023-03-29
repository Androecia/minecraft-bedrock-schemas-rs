/* Raw contents of play_dead.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.play_dead",
  "description": "Allows the mob to play dead when attacked by other entities. When playing dead, other entities will not target this mob.",
  "type": "object",
  "title": "Play Dead",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "apply_regeneration": {
      "title": "Apply Regeneration",
      "type": "boolean",
      "default": true,
      "description": "Whether the mob will receive the regeneration effect while playing dead."
    },
    "duration": {
      "title": "Duration",
      "type": "number",
      "default": 1.0,
      "description": "The amount of time the mob will remain playing dead (in seconds)."
    },
    "filters": {
      "title": "Filters",
      "description": "The list of other triggers that are required for the mob to activate play dead.",
      "$ref": "../../filters/filters.json"
    },
    "force_below_health": {
      "title": "Force Below Health",
      "type": "integer",
      "default": 0,
      "description": "The amount of health at which damage will cause the mob to play dead."
    },
    "random_start_chance": {
      "title": "Random Start Chance",
      "type": "number",
      "default": 1.0,
      "description": "The likelihood of this goal starting upon taking damage."
    },
    "random_damage_range": {
      "title": "Random Damage Range",
      "description": "The range of damage that may cause the goal to start depending on randomness. Damage taken below the min will never cause the goal to start. Damage taken above the max will always cause the goal to start.",
      "type": "array",
      "items": [
        { "title": "Minimum", "description": "Minimum.", "type": "integer", "minimum": 0 },
        { "title": "Maximum", "description": "Maximum.", "type": "integer", "minimum": 0 }
      ]
    },
    "damage_sources": {
      "title": "Damage Sources",
      "description": "The list of Entity Damage Sources that will cause this mob to play dead.",
      "default": ["all"],
      "oneOf": [
        {
          "type": "string",
          "$ref": "../../../../general/entity/damage_source.json"
        },
        {
          "type": "array",
          "items": {
            "title": "Damage Source",
            "$ref": "../../../../general/entity/damage_source.json"
          }
        }
      ]
    }
  },
  "examples": [
    {
      "duration": 0.0,
      "force_below_health": 0.0,
      "random_start_chance": 0.0,
      "random_damage_range": [],
      "apply_regeneration": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayDead {}
