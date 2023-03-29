/* Raw contents of moon_intensity.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.moon_intensity",
  "type": "object",
  "title": "Moon Intensity",
  "description": "Compares the current moon intensity with a float value in the range (0.0, 1.0)",
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
      "description": "A floating point value.",
      "type": "number",
      "minimum": 0,
      "maximum": 1,
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "moon_intensity",
      "value": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoonIntensity {}
