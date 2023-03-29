/* Raw contents of vibration_listener.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.vibration_listener",
  "type": "object",
  "title": "Vibration Listener",
  "additionalProperties": false,
  "required": [],
  "properties": {},
  "description": "This entity will respond to vibrations."
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VibrationListener {}
