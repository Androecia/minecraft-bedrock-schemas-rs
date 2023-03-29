/* Raw contents of eat_carried_item.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.behavior.eat_carried_item",
  "type": "object",
  "title": "Eat Carried Item",
  "description": "If the mob is carrying a food item, the mob will eat it and the effects will be applied to the mob.",
  "additionalProperties": false,
  "properties": {
    "priority": {
      "$ref": "types/priority.json"
    },
    "delay_before_eating": {
      "type": "number",
      "description": "Time in seconds the mob should wait before eating the item.",
      "title": "Delay Before Eating"
    }
  },
  "examples": [
    {
      "delay_before_eating": 0.0
    }
  ]
}*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EatCarriedItem {}
