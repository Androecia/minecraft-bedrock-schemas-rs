/* Raw contents of is_snow_covered.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_snow_covered",
  "type": "object",
  "title": "Is Snow Covered",
  "description": "Tests whether the Subject is in an area with snow cover.",
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
      "title": "Value",
      "description": "True or false.",
      "type": "boolean",
      "default": true
    }
  },
  "examples": [
    {
      "test": "is_snow_covered",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsSnowCovered {}
