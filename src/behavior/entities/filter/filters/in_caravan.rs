/* Raw contents of in_caravan.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.in_caravan",
  "type": "object",
  "title": "In Caravan",
  "description": "Returns true if the subject entity is in a caravan.",
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true if the subject entity is in a caravan."
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
      "test": "in_caravan",
      "value": true
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InCaravan {}
