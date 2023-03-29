/* Raw contents of mob_effect.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.mob_effect",
  "type": "object",
  "title": "Mob Effect",
  "description": "A component that applies a mob effect to entities that get within range.",
  "additionalProperties": false,
  "properties": {
    "cooldown_time": {
      "title": "Cooldown Time",
      "type": "integer",
      "default": 0,
      "description": "Time in seconds to wait between each application of the effect."
    },
    "effect_range": {
      "title": "Effect Range",
      "type": "number",
      "default": 0.2,
      "description": "How close a hostile entity must be to have the mob effect applied."
    },
    "effect_time": {
      "title": "Effect Time",
      "type": "integer",
      "default": 10,
      "description": "How long the applied mob effect lasts in seconds."
    },
    "entity_filter": {
      "$ref": "../../filters/filters.json",
      "title": "Entity Filter",
      "description": "Filter to use for conditions."
    },
    "mob_effect": {
      "title": "Mob Effect",
      "type": "string",
      "default": "",
      "description": "The mob effect that is applied to entities that enter this entities effect range."
    }
  },
  "examples": [
    {
      "effect_range": 0.2,
      "effect_time": 10,
      "mob_effect": ""
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MobEffect {}
