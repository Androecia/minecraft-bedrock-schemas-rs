/* Raw contents of inventory.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.inventory",
  "description": "Defines this entity's inventory properties.",
  "type": "object",
  "title": "Inventory",
  "additionalProperties": false,
  "properties": {
    "additional_slots_per_strength": {
      "type": "integer",
      "default": 0,
      "description": "Number of slots that this entity can gain per extra strength.",
      "title": "Additional Slots Per Strength"
    },
    "can_be_siphoned_from": {
      "type": "boolean",
      "default": false,
      "description": "If true, the contents of this inventory can be removed by a hopper.",
      "title": "Can Be Siphoned From"
    },
    "container_type": {
      "type": "string",
      "default": "none",
      "description": "Type of container this entity has. Can be horse, minecart_chest, chest_boat, minecart_hopper, inventory, container or hopper",
      "title": "Container Type",
      "enum": ["horse", "minecart_chest", "chest_boat", "minecart_hopper", "inventory", "container", "hopper"]
    },
    "inventory_size": {
      "type": "integer",
      "default": 5,
      "description": "Number of slots the container has.",
      "title": "Inventory Size"
    },
    "private": {
      "type": "boolean",
      "default": false,
      "description": "If true, only the entity can access the inventory.",
      "title": "Private"
    },
    "restrict_to_owner": {
      "type": "boolean",
      "default": false,
      "description": "If true, the entity's inventory can only be accessed by its owner or itself.",
      "title": "Restrict To Owner"
    }
  },
  "examples": [
    {
      "additional_slots_per_strength": 0,
      "can_be_siphoned_from": false,
      "container_type": "none",
      "inventory_size": 5,
      "private": false,
      "restrict_to_owner": false
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inventory {}
