/* Raw contents of equip_item.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.equip_item",
  "type": "object",
  "title": "Equip Item",
  "description": "The entity puts on the desired equipment.",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipItem {}
