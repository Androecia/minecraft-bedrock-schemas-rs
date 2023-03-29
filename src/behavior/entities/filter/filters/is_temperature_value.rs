/* Raw contents of is_temperature_value.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_temperature_value",
  "type": "object",
  "title": "Is Temperature Value",
  "description": "Tests the current temperature against a provided value in the range (0.0, 1.0) where 0.0f is the coldest temp and 1.0f is the hottest.",
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
      "description": "The Biome temperature value to compare with.",
      "type": "number",
      "minimum": 0.0,
      "maximum": 1.0,
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "is_temperature_value",
      "value": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsTemperatureValue {}
