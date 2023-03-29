/* Raw contents of silverfish_merge_with_stone.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.silverfish_merge_with_stone",
  "description": "Allows the mob to go into stone blocks like Silverfish do. Currently it can only be used by Silverfish.",
  "type": "object",
  "title": "Silverfish Merge With Stone",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SilverfishMergeWithStone {}
