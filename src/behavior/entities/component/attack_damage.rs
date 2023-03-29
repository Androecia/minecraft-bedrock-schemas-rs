/* Raw contents of attack_damage.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.attack_damage",
  "type": "object",
  "title": "Attack Damage",
  "additionalProperties": false,
  "required": ["value"],
  "properties": {
    "value": {
      "type": "number",
      "description": "UNDOCUMENTED: value.",
      "title": "Value"
    }
  },
  "description": "UNDOCUMENTED.",
  "examples": [
    {
      "value": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttackDamage {}
