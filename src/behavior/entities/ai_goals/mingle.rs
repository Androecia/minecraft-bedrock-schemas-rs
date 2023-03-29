/* Raw contents of mingle.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.mingle",
  "description": "Allows an entity to go to the village bell and mingle with other entities.",
  "type": "object",
  "title": "Mingle",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "cooldown_time": {
      "type": "number",
      "default": 0,
      "description": "Time in seconds the mob has to wait before using the goal again.",
      "title": "Cooldown Time"
    },
    "duration": {
      "type": "number",
      "default": 1,
      "description": "Amount of time in seconds that the entity will chat with another entity.",
      "title": "Duration"
    },
    "mingle_distance": {
      "type": "number",
      "default": 2.0,
      "description": "The distance from its partner that this entity will mingle. If the entity type is not the same as the entity, this value needs to be identical on both entities.",
      "title": "Mingle Distance"
    },
    "mingle_partner_type": {
      "description": "The entity type that this entity is allowed to mingle with.",
      "title": "Mingle Partner Type",
      "oneOf": [
        {
          "type": "array",
          "items": {
            "description": "The entity type that this entity is allowed to mingle with.",
            "type": "string"
          }
        },
        {
          "type": "string"
        }
      ]
    }
  },
  "examples": [
    {
      "cooldown_time": 0,
      "duration": 1,
      "mingle_distance": 2.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mingle {}
