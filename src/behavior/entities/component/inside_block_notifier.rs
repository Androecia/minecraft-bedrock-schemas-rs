/* Raw contents of inside_block_notifier.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.inside_block_notifier",
  "type": "object",
  "title": "Inside Block Notifier",
  "additionalProperties": false,
  "description": "Verifies whether the entity is inside any of the listed blocks.",
  "required": [],
  "properties": {
    "block_list": {
      "title": "Block List",
      "description": "List of blocks, with certain block states, that we are monitoring to see if the entity is inside.",
      "type": "array",
      "items": {
        "title": "Block",
        "description": "A of block, with certain block states, that we are monitoring to see if the entity is inside.",
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "block": {
            "$ref": "../../../../general/block_definition.json"
          },
          "entered_block_event": {
            "title": "Entered Block Event",
            "description": "Event to run when this mob enters a valid block.",
            "$comment": "UNDOCUMENTED",
            "$ref": "../types/event_object.json"
          },
          "exited_block_event": {
            "title": "Exited Block Event",
            "description": "Event to run when this mob leaves a valid block.",
            "$comment": "UNDOCUMENTED",
            "$ref": "../types/event_object.json"
          }
        }
      }
    }
  },
  "examples": [
    {
      "block_list": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsideBlockNotifier {}
