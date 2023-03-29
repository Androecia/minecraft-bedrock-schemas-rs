/* Raw contents of random_chance.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.random_chance",
  "type": "object",
  "title": "Random Chance",
  "description": "Returns true if the random chance rolls 0 out of a specified Maximum range.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test",
      "description": "The test property."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "An integer value.",
      "type": "integer",
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "random_chance",
      "value": 0
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomChance {}
