/* Raw contents of sound_volume.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.sound_volume",
  "type": "object",
  "title": "Sound Volume",
  "additionalProperties": false,
  "description": "Sets the entity's base volume for sound effects.",
  "required": [],
  "properties": {
    "value": {
      "type": "number",
      "default": 1.0,
      "description": "The value of the volume the entity uses for sound effects.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "value": 1.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoundVolume {}
