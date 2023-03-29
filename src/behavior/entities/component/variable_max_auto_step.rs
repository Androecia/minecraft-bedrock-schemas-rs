/* Raw contents of variable_max_auto_step.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.variable_max_auto_step",
  "type": "object",
  "title": "Variable Max Auto Step",
  "additionalProperties": false,
  "description": "Entities with this component will have a maximum auto step height that is different depending on wether they are on a block that prevents jumping. Incompatible with \"runtime_identifier\": \"minecraft:horse\".",
  "required": [],
  "properties": {
    "base_value": {
      "title": "base value",
      "type": "number",
      "default": 0.5625,
      "description": "The maximum auto step height when on any other block."
    },
    "jump_prevented_value": {
      "title": "jump prevented value",
      "type": "number",
      "default": 0.5625,
      "description": "The maximum auto step height when on a block that prevents jumping."
    }
  },
  "examples": [
    {
      "value": 0.0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableMaxAutoStep {}
