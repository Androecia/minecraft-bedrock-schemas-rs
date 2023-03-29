/* Raw contents of ambient_sound_interval.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.ambient_sound_interval",
  "additionalProperties": false,
  "type": "object",
  "title": "Ambient Sound Interval",
  "description": "Sets the entity's delay between playing its ambient sound.",
  "properties": {
    "event_name": {
      "type": "string",
      "default": "ambient",
      "description": "Level sound event to be played as the ambient sound.",
      "title": "Event Name"
    },
    "event_names": {
      "type": "array",
      "description": "List of dynamic level sound events, with conditions for choosing between them. Evaluated in order, first one wins. If none evaluate to true, 'event_name' will take precedence.",
      "items": {
        "type": "object",
        "properties": {
          "condition": {
            "type": "string",
            "description": "The condition that must be satisfied to select the given ambient sound.",
            "title": "Condition"
          },
          "event_name": {
            "$ref": "../../../../general/sound_event.json",
            "description": "Level sound event to be played as the ambient sound.",
            "title": "Event Name"
          }
        }
      }
    },
    "range": {
      "title": "Range",
      "type": "number",
      "default": 16.0,
      "description": "Maximum time in seconds to randomly add to the ambient sound delay time."
    },
    "value": {
      "title": "Value",
      "type": "number",
      "default": 8.0,
      "description": "Minimum time in seconds before the entity plays its ambient sound again."
    }
  },
  "examples": [
    {
      "event_name": "ambient",
      "range": 16,
      "value": 8
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbientSoundInterval {}