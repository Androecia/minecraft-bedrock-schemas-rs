/* Raw contents of bool_property.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.bool_property",
  "type": "object",
  "title": "Bool Property",
  "description": "Returns true when the bool actor property matches the value provided.",
  "required": ["value", "domain"],
  "examples": [{ "test": "bool_property", "value": true }],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true when the bool actor property matches the value provided."
    },
    "domain": {
      "description": "(Required) The property name to look for",
      "title": "Operator",
      "type": "string"
    },
    "operator": {
      "$ref": "./types/operator.json",
      "description": "(Optional) The comparison to apply with `value`.",
      "default": "equals",
      "title": "Operator"
    },
    "subject": {
      "$ref": "./types/subject.json",
      "description": "(Optional) The subject of this filter test.",
      "default": "self",
      "title": "Subject"
    },
    "value": {
      "type": "boolean",
      "default": true,
      "title": "Value",
      "description": "true or false."
    }
  }
}
*/
use serde::{Deserialize, Serialize};
use super::super::{Operator, Subject};


/// Returns true when the bool actor property matches the value provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoolProperty {

    /// (Required) The property name to look for
    pub domain: String,

    /// (Optional) The comparison to apply with `value`.

    pub operator: Option<Operator>,

    /// (Optional) The subject of this filter test.
    pub subject: Subject,

    /// true or false.
    pub value: bool,
}
