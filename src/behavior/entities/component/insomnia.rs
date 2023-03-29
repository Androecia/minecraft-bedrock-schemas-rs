/* Raw contents of insomnia.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.insomnia",
  "description": "Adds a timer since last rested to see if phantoms should spawn.",
  "type": "object",
  "title": "Insomnia",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "days_until_insomnia": {
      "type": "number",
      "default": 3,
      "description": "Number of days the mob has to stay up until the insomnia effect begins.",
      "title": "Days Until Insomnia"
    }
  },
  "examples": [
    {
      "days_until_insomnia": 3
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Insomnia {}
