/* Raw contents of is_dyeable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.is_dyeable",
  "description": "Allows dyes to be used on this entity to change its color.",
  "type": "object",
  "title": "Is Dyeable",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "interact_text": {
      "type": "string",
      "description": "The text that will display when interacting with this entity with a dye when playing with Touch-screen controls.",
      "title": "Interact Text"
    }
  },
  "examples": [
    {
      "interact_text": "example"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsDyeable {}
