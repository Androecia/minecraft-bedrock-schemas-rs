/* Raw contents of item_hopper.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.item_hopper",
  "type": "object",
  "title": "Item Hopper",
  "additionalProperties": false,
  "required": [],
  "properties": {},
  "description": "Determines that this entity is an item hopper."
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemHopper {}
