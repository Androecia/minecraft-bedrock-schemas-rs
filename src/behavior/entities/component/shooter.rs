/* Raw contents of shooter.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.shooter",
  "description": "Defines the entity's ranged attack behavior.",
  "type": "object",
  "title": "Shooter",
  "additionalProperties": false,
  "properties": {
    "aux_val": {
      "type": "integer",
      "title": "Aux Val",
      "default": -1,
      "description": "ID of the Potion effect to be applied on hit."
    },
    "def": {
      "title": "Def",
      "type": "string",
      "examples": ["minecraft:arrow", "minecraft:small_fireball", "minecraft:thrown_trident"],
      "description": "Actor definition to use as projectile for the ranged attack. The actor definition must have the projectile component to be able to be shot as a projectile"
    },
    "type": {
      "title": "Type",
      "type": "string",
      "description": "UNDOCUMENTED.",
      "$comment": "UNDOCUMENTED",
      "examples": ["dragonfireball"]
    }
  },
  "examples": [{ "def": "minecraft:small_fireball" }]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shooter {}
