/* Raw contents of moon_phase.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.moon_phase",
  "type": "object",
  "title": "Moon Phase",
  "description": "Compares the current moon phase with an integer value in the range (0, 7).",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "The test property."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "type": "integer",
      "description": "An integer value.",
      "minimum": 0,
      "maximum": 7,
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "moon_phase",
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoonPhase {}
