/* Raw contents of vibration_damper.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.vibration_damper",
  "type": "object",
  "title": "Vibration Damper",
  "additionalProperties": false,
  "required": [],
  "properties": {},
  "description": "Vibrations emitted by this entity will be ignored."
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VibrationDamper {}
