/* Raw contents of equippable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.equippable",
  "type": "object",
  "title": "Equippable",
  "additionalProperties": false,
  "description": "Defines an entity's behavior for having items equipped to it.",
  "properties": {
    "slots": {
      "description": "List of slots and the item that can be equipped.",
      "type": "array",
      "title": "Slots",
      "items": {
        "description": "A slot and the item that can be equipped.",
        "title": "Slots",
        "type": "object",
        "properties": {
          "slot": {
            "type": "integer",
            "default": 0,
            "description": "The slot number of this slot.",
            "title": "Slot"
          },
          "accepted_items": {
            "type": "array",
            "description": "The list of items that can go in this slot.",
            "title": "Accepted Items",
            "items": {
              "type": "string",
              "description": "A item name.",
              "$ref": "../../../../general/item/descriptor.json",
              "title": "Accepted Items"
            }
          },
          "item": {
            "$ref": "../../../../general/item/descriptor.json",
            "description": "Identifier of the item that can be equipped for this slot.",
            "title": "Item"
          },
          "interact_text": {
            "type": "string",
            "description": "Text to be displayed when the entity can be equipped with this item when playing with Touch-screen controls.",
            "title": "Interact Text"
          },
          "on_equip": {
            "$ref": "../types/event_object.json",
            "description": "Event to trigger when this entity is equipped with this item.",
            "title": "On Equip"
          },
          "on_unequip": {
            "$ref": "../types/event_object.json",
            "description": "Event to trigger when this item is removed from this entity.",
            "title": "On Unequip"
          }
        }
      }
    }
  },
  "examples": [
    {
      "slots": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equippable {}
