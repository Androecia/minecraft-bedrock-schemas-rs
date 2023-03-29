/* Raw contents of celebrate_hunt.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.celebrate_hunt",
  "type": "object",
  "title": "Celebrate Hunt",
  "description": "Specifies hunt celebration behavior.",
  "additionalProperties": false,
  "properties": {
    "broadcast": {
      "type": "boolean",
      "default": true,
      "description": "If true, celebration will be broadcasted to other entities in the radius.",
      "title": "Broadcast"
    },
    "celebration_targets": {
      "$ref": "../../filters/filters.json",
      "description": "The list of conditions that target of hunt must satisfy to initiate celebration.",
      "title": "Celeberation Targets"
    },
    "celebrate_sound": {
      "$ref": "../../../../general/sound_event.json",
      "default": "",
      "description": "The sound event to play when the mob is celebrating.",
      "title": "Celebrate Sound"
    },
    "duration": {
      "type": "integer",
      "default": 4,
      "description": "Duration, in seconds, of celebration.",
      "title": "Duration"
    },
    "radius": {
      "type": "number",
      "default": 16,
      "description": "If broadcast is enabled, specifies the radius in which it will notify other entities for celebration.",
      "title": "Radius"
    },
    "sound_interval": {
      "default": 0,
      "description": "The range of time in seconds to randomly wait before playing the sound again.",
      "title": "Sound Interval",
      "oneOf": [
        {
          "type": "array",
          "items": [
            {
              "type": "number",
              "title": "Maximum"
            },
            {
              "type": "number",
              "title": "Maximum"
            }
          ]
        },
        {
          "type": "number"
        },
        {
          "type": "object",
          "additionalProperties": false,
          "properties": {
            "range_min": {
              "type": "number",
              "title": "Minimum",
              "description": "Minimum."
            },
            "range_max": {
              "type": "number",
              "title": "Maximum",
              "description": "Maximum."
            }
          }
        }
      ]
    }
  },
  "examples": [
    {
      "broadcast": true,
      "celebrate_sound": "",
      "duration": 4,
      "radius": 16
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CelebrateHunt {}
