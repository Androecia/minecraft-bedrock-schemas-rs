/* Raw contents of ocelotattack.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.ocelotattack",
  "type": "object",
  "title": "Ocelotattack",
  "additionalProperties": false,
  "description": "Can only be used by the Ocelot. Allows it to perform the sneak and pounce attack.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },

    "cooldown_time": {
      "title": "Cooldown Time",
      "type": "number",
      "default": 1,
      "description": "Time (in seconds) between attacks."
    },
    "max_distance": {
      "title": "Max Distance",
      "type": "number",
      "default": 15,
      "description": "Max distance from the target, this entity will use this attack behavior."
    },
    "max_sneak_range": {
      "title": "Max Sneak Range",
      "type": "number",
      "default": 15,
      "description": "Max distance from the target, this entity starts sneaking."
    },
    "max_sprint_range": {
      "title": "Max Sprint Range",
      "type": "number",
      "default": 4,
      "description": "Max distance from the target, this entity starts sprinting (sprinting takes priority over sneaking)."
    },
    "reach_multiplier": {
      "title": "Reach Multiplier",
      "type": "number",
      "default": 2,
      "description": "Used with the base size of the entity to determine minimum target-distance before trying to deal attack damage."
    },
    "sneak_speed_multiplier": {
      "title": "Sneak Speed Multiplier",
      "type": "number",
      "default": 0.6,
      "description": "Modifies the attacking entity's movement speed while sneaking."
    },
    "sprint_speed_multiplier": {
      "title": "Sprint Speed Multiplier",
      "type": "number",
      "default": 1.33,
      "description": "Modifies the attacking entity's movement speed while sprinting."
    },
    "walk_speed_multiplier": {
      "title": "Walk Speed Multiplier",
      "type": "number",
      "default": 0.8,
      "description": "Modifies the attacking entity's movement speed when not sneaking or sprinting, but still within attack range."
    },
    "x_max_rotation": {
      "title": "X Max Rotation",
      "type": "number",
      "default": 30,
      "description": "Maximum rotation (in degrees), on the X-axis, this entity can rotate while trying to look at the target."
    },
    "y_max_head_rotation": {
      "title": "Y Max Head Rotation",
      "type": "number",
      "default": 30,
      "description": "Maximum rotation (in degrees), on the Y-axis, this entity can rotate its head while trying to look at the target."
    }
  },
  "examples": [
    {
      "walk_speed_multiplier": 1.0,
      "sprint_speed_multiplier": 1.0,
      "sneak_speed_multiplier": 1.0,
      "cooldown_time": 0.0,
      "x_max_rotation": 0.0,
      "y_max_head_rotation": 0.0,
      "max_distance": 0.0,
      "max_sneak_range": 0.0,
      "max_sprint_range": 0.0,
      "reach_multiplier": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ocelotattack {}
