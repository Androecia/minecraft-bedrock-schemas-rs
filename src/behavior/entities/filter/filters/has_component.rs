/* Raw contents of has_component.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_component",
  "type": "object",
  "title": "Has Component",
  "description": "Returns true when the subject entity contains the named component.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true when the subject entity contains the named component."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "type": "string",
      "description": "(Required) The component name to look for.",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "has_component",
      "value": "minecraft:explode"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasComponent {}
