/* Raw contents of swoop_attack.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.swoop_attack",
  "description": "Allows the mob to move to attack a target. The goal ends if it has a horizontal collision or gets hit. Built to be used with flying mobs.",
  "type": "object",
  "title": "Swoop Attack",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "damage_reach": {
      "title": "Damage Reach",
      "type": "number",
      "default": 0.2,
      "description": "Added to the base size of the entity, to determine the target's maximum allowable distance, when trying to deal attack damage."
    },
    "delay_range": {
      "$ref": "../types/range_number_type.json",
      "default": [[10.0, 20.0]],
      "description": "Minimum and maximum cooldown time-range (in seconds) between each attempted swoop attack.",
      "title": "Delay Range"
    }
  },
  "examples": [
    {
      "damage_reach": 0.2,
      "delay_range": [10.0, 20.0]
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwoopAttack {}
