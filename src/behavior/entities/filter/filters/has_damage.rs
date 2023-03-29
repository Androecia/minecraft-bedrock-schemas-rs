/* Raw contents of has_damage.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_damage",
  "type": "object",
  "title": "Has Damage",
  "description": "Returns true when the subject entity receives the named damage type. has_damage can also use subject and operator parameters but they are optional.",
  "required": ["value"],
  "examples": [{ "test": "has_damage", "value": "fatal" }],
  "properties": {
    "test": { "type": "string", "title": "Test Property", "description": "Returns true when the subject entity receives the named damage type." },
    "operator": { "$ref": "./types/operator.json" },
    "subject": { "$ref": "./types/subject.json" },
    "value": { "type": "string", "description": "The Damage type to test.", "$ref": "../../../../general/entity/damage_source.json", "title": "Value" }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasDamage {}
