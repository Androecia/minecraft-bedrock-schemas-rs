/* Raw contents of owner_hurt_target.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.owner_hurt_target",
  "type": "object",
  "title": "Owner Hurt Target",
  "additionalProperties": false,
  "description": "Allows the mob to target a mob that is hurt by their owner.",
  "required": [],
  "properties": {
    "priority": {
      "$ref": "./types/priority.json"
    },
    "entity_types": {
      "$ref": "../types/entity_types.json",
      "description": "List of entity types that this entity can target if the potential target is hurt by this mob's owner.",
      "title": "Entity TYpes"
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnerHurtTarget {}
