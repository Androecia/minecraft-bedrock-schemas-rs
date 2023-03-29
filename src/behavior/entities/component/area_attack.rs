/* Raw contents of area_attack.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.area_attack",
  "type": "object",
  "title": "Area Attack",
  "description": "A component that does damage to entities that get within range.",
  "additionalProperties": false,
  "properties": {
    "cause": {
      "title": "Cause",
      "$ref": "../../../../general/entity/damage_source.json",
      "type": "string",
      "description": "The type of damage that is applied to entities that enter the damage range."
    },
    "damage_cooldown": {
      "title": "damage cooldown",
      "type": "number",
      "default": 0,
      "description": "Attack cooldown (in seconds) for how often this entity can attack a target."
    },
    "damage_per_tick": {
      "type": "integer",
      "default": 2,
      "description": "How much damage per tick is applied to entities that enter the damage range.",
      "title": "Damage Per Tick"
    },
    "damage_range": {
      "type": "number",
      "default": 0.2,
      "description": "How close a hostile entity must be to have the damage applied.",
      "title": "Damage Range"
    },
    "entity_filter": {
      "$ref": "../../filters/filters.json",
      "description": "The set of entities that are valid to apply the damage to when within range.",
      "title": "Entity Filter"
    },
    "play_attack_sound": {
      "title": "play attack sound",
      "type": "boolean",
      "default": 4.94066e-324,
      "description": "If the entity should play their attack sound when attacking a target."
    }
  },
  "examples": [
    {
      "damage_per_tick": 2,
      "damage_range": 0.2,
      "cause": "example"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AreaAttack {}
