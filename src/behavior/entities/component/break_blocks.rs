/* Raw contents of break_blocks.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.break_blocks",
  "type": "object",
  "title": "Break Blocks",
  "additionalProperties": false,
  "description": "Specifies the blocks that this entity can break as it moves around.",
  "required": [],
  "properties": {
    "breakable_blocks": {
      "type": "array",
      "title": "Breakable Blocks",
      "description": "A list of the blocks that can be broken as this entity moves around.",
      "items": {
        "$ref": "../../../../general/blocks_item.json"
      }
    }
  },
  "examples": [
    {
      "breakable_blocks": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakBlocks {}
