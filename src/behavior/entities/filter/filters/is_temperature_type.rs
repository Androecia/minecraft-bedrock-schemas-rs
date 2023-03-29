/* Raw contents of is_temperature_type.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_temperature_type",
  "type": "object",
  "title": "Is Target",
  "description": "Tests whether the current temperature is a given type.",
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
      "description": "The Biome temperature catagory to test.",
      "type": "string",
      "enum": ["cold", "mild", "ocean", "warm"],
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "is_temperature_type",
      "value": "cold"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsTemperatureType {}
