/* Raw contents of knockback_roar.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.knockback_roar",
  "description": "Allows the mob to perform a damaging knockback that affects all nearby entities.",
  "type": "object",
  "title": "Knockback Roar",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "attack_time": {
      "title": "Attack Time",
      "type": "number",
      "default": 0.5,
      "description": "The delay after which the knockback occurs (in seconds)."
    },
    "cooldown_time": {
      "title": "Cooldown Time",
      "type": "number",
      "default": 0,
      "description": "Time in seconds the mob has to wait before using the goal again."
    },
    "damage_filters": {
      "$ref": "../../filters/filters.json",
      "description": "The list of conditions another entity must meet to be a valid target to apply damage to.",
      "title": "Damage Filters"
    },
    "duration": {
      "type": "number",
      "default": 1,
      "description": "The duration of the roar (in seconds).",
      "title": "Duration"
    },
    "knockback_damage": {
      "type": "integer",
      "default": 6,
      "description": "The damage dealt by the knockback roar.",
      "title": "Knockback Damage"
    },
    "knockback_strength": {
      "type": "integer",
      "default": 4,
      "description": "The strength of the knockback.",
      "title": "Knockback Strength"
    },
    "knockback_filters": {
      "$ref": "../../filters/filters.json",
      "description": "The list of conditions another entity must meet to be a valid target to apply knockback to.",
      "title": "Knockback Filters"
    },
    "knockback_horizontal_strength": {
      "type": "integer",
      "default": 4,
      "description": "The strength of the horizontal knockback.",
      "title": "Knockback Horizontal Strength"
    },
    "knockback_range": {
      "type": "integer",
      "default": 4,
      "description": "The radius (in blocks) of the knockback effect.",
      "title": "Knockback Range"
    },
    "knockback_vertical_strength": {
      "type": "integer",
      "default": 4,
      "description": "The strength of the vertical knockback.",
      "title": "Knockback Vertical Strength"
    },
    "knockback_height_cap": {
      "title": "Knockback Height Cap",
      "type": "number",
      "default": 0.4,
      "description": "The maximum height for vertical knockback."
    },
    "track_target": {
      "type": "boolean",
      "default": false,
      "description": "If true, this mob will chase after the target as long as it's a valid target.",
      "title": "Track Target"
    },
    "on_roar_end": {
      "$ref": "../types/event.json",
      "description": "Event that is triggered when the roar ends.",
      "title": "On Roar End"
    }
  },
  "examples": [
    {
      "attack_time": 0.5,
      "cooldown_time": 0,
      "duration": 1,
      "knockback_damage": 6,
      "knockback_horizontal_strength": 4,
      "knockback_range": 4,
      "knockback_vertical_strength": 4,
      "track_target": false,
      "knockback_strength": 4
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnockbackRoar {}
