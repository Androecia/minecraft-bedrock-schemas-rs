/* Raw contents of weather.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.filters.weather",
  "type": "object",
  "title": "Weather",
  "description": "Tests for the current weather state the entity is experiencing.",
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
      "description": "The Family name to look for.",
      "type": "string",
      "title": "Value",
      "examples": ["clear", "thunderstorm"]
    }
  },
  "examples": [
    {
      "test": "weather",
      "value": "clear"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weather {}
