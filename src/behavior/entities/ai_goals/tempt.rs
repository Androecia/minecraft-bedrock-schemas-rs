/* Raw contents of tempt.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.tempt",
  "description": "Allows an entity to be tempted by a set item.",
  "type": "object",
  "title": "Tempt",
  "additionalProperties": false,
  "properties": {
    "priority": { "$ref": "types/priority.json" },
    "speed_multiplier": { "$ref": "types/speed_multiplier.json" },
    "can_get_scared": {
      "type": "boolean",
      "default": false,
      "description": "If true, the mob can stop being tempted if the player moves too fast while close to this mob.",
      "title": "Can Get Scared"
    },
    "can_tempt_while_ridden": {
      "type": "boolean",
      "default": false,
      "title": "Can Tempt While Ridden",
      "description": "If true, the mob can be tempted even if it has a passenger (i.e. if being ridden)."
    },
    "can_tempt_vertically": {
      "type": "boolean",
      "default": false,
      "title": "Can Tempt Vertically",
      "description": "If true, vertical distance to the player will be considered when tempting."
    },
    "items": {
      "type": "array",
      "title": "Items",
      "description": "List of items this mob is tempted by.",
      "items": {
        "$ref": "../../../../general/item/descriptor.json"
      }
    },
    "sound_interval": {
      "description": "Range of random ticks to wait between tempt sounds.",
      "title": "Sound Interval",
      "oneOf": [
        {
          "type": "number",
          "minimum": 0
        },
        {
          "items": [
            { "type": "integer", "minimum": 0, "title": "Minimum" },
            { "type": "integer", "minimum": 0, "title": "Maximum" }
          ]
        }
      ]
    },
    "tempt_sound": {
      "$ref": "../../../../general/sound_event.json",
      "description": "Sound to play while the mob is being tempted.",
      "title": "Tempt Sound"
    },
    "within_radius": {
      "type": "number",
      "default": 0,
      "description": "Distance in blocks this mob can get tempted by a player holding an item they like.",
      "title": "Within Radius"
    }
  },
  "examples": [
    {
      "can_get_scared": false,
      "can_tempt_while_ridden": true,
      "can_tempt_vertically": true,
      "items": [],
      "within_radius": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tempt {}
