/* Raw contents of balloonable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.balloonable",
  "additionalProperties": false,
  "type": "object",
  "title": "Balloonable",
  "required": [],
  "properties": {
    "mass": {
      "type": "number",
      "description": "UNDOCUMENTED: mass.",
      "title": "Mass"
    }
  },
  "description": "UNDOCUMENTED.",
  "examples": [
    {
      "mass": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balloonable {}
