/* Raw contents of is_block.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_block",
  "type": "object",
  "title": "Is Block",
  "description": "Returns true when the block has the given name.",
  "required": ["value"],
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
      "description": "The Family name to look for.",
      "type": "string",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "is_block",
      "subject": "block",
      "value": "minecraft:sweet_berry_bush"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsBlock {}
