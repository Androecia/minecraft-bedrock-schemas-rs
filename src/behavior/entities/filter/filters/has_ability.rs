/* Raw contents of has_ability.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.has_ability",
  "type": "object",
  "title": "Has Ability",
  "description": "Returns true when the subject entity has the named ability.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Returns true when the subject entity has the named ability."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "type": "string",
      "description": "(Required) The Ability type to test.",
      "enum": ["flySpeed", "flying", "instabuild", "invulnerable", "lightning", "mayfly", "mute", "noclip", "walkSpeed", "worldbuilder"],
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "has_ability",
      "value": "flySpeed"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasAbility {}
