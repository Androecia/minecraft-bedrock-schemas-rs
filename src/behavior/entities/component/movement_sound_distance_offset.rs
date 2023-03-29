/* Raw contents of movement_sound_distance_offset.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.movement_sound_distance_offset",
  "type": "object",
  "title": "Movement Sound Distance Offset",
  "description": "Sets the offset used to determine the next step distance for playing a movement sound.",
  "required": ["value"],
  "properties": {
    "value": {
      "type": "number",
      "default": 1.0,
      "description": "The higher the number, the less often the movement sound will be played.",
      "title": "Value"
    }
  },
  "additionalProperties": false,
  "examples": [{ "value": 1.0 }, { "value": 0.5 }, { "value": 1.5 }]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovementSoundDistanceOffset {}
