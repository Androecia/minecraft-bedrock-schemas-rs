/* Raw contents of is_difficulty.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.is_difficulty",
  "type": "object",
  "title": "Is Difficulty",
  "description": "Tests the current difficulty level of the game.",
  "required": ["value"],
  "properties": {
    "test": {
      "type": "string",
      "title": "Test Property",
      "description": "Tests the current difficulty level of the game."
    },
    "operator": {
      "$ref": "./types/operator.json"
    },
    "subject": {
      "$ref": "./types/subject.json"
    },
    "value": {
      "description": "The game's difficulty level to test.",
      "type": "string",
      "enum": ["easy", "hard", "normal", "peaceful"],
      "title": "Value"
    }
  },
  "examples": [
    {
      "test": "is_difficulty",
      "value": "easy"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsDifficulty {}
