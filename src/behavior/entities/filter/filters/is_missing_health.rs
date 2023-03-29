/* Raw contents of is_missing_health.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_missing_health",
  "type": "object",
  "title": "In Nether",
  "description": "Tests if the subject is not at full health.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test",
      "description": "The test property."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "True or false.",
      "type": "boolean",
      "default": true,
      "title": "Value"
    }
  },
  "examples": [{ "test": "is_missing_health", "value": true }]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsMissingHealth {}
