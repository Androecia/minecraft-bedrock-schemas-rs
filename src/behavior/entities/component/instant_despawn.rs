/* Raw contents of instant_despawn.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.instant_despawn",
  "type": "object",
  "title": "Instant Despawn",
  "description": "Despawns the Actor immediately.",
  "additionalProperties": false,
  "properties": {
    "remove_child_entities": {
      "type": "boolean",
      "default": false,
      "description": "If true, all entities linked to this entity in a child relationship (eg. leashed) will also be despawned.",
      "title": "Remove Child Entities"
    }
  },
  "examples": [
    {
      "remove_child_entities": false
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantDespawn {}
