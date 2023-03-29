/* Raw contents of hurt_on_condition.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.hurt_on_condition",
  "type": "object",
  "title": "Hurt On Condition",
  "description": "Defines a set of conditions under which an entity should take damage.",
  "additionalProperties": false,
  "properties": {
    "damage_conditions": {
      "type": "array",
      "title": "Damage Conditions",
      "description": "An array of conditions used to compare the event to.",
      "items": {
        "type": "object",
        "title": "Damage Condition",
        "description": "A condition used to compare the event to.",
        "additionalProperties": false,
        "properties": {
          "filters": {
            "$ref": "../../filters/filters.json"
          },
          "cause": {
            "type": "string",
            "title": "Cause",
            "description": "Damage cause.",
            "$ref": "../../../../general/entity/damage_source.json"
          },
          "damage_per_tick": {
            "type": "integer",
            "title": "Damage Per Tick",
            "description": "Amount of damage done each tick that the conditions are met."
          }
        }
      }
    }
  },
  "examples": [
    {
      "damage_conditions": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HurtOnCondition {}
