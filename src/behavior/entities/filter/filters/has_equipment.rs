/* Raw contents of has_equipment.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_equipment",
  "type": "object",
  "title": "Has Equipment",
  "description": "Tests for the presence of a named item in the designated slot of the subject entity.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "const": "has_equipment",
      "description": "Tests for the presence of a named item in the designated slot of the subject entity.",
      "title": "Test"
    },
    "domain": {
      "description": "The equipment location to test.",
      "default": "any",
      "enum": ["any", "armor", "feet", "hand", "head", "leg", "torso"],
      "title": "Domain"
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "The item name to look for.",
      "type": "string",
      "$ref": "../../../../general/item/identifier.json",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "has_equipment",
      "value": "example"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasEquipment {}
