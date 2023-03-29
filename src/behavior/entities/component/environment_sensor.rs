/* Raw contents of environment_sensor.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.environment_sensor",
  "title": "Environment Sensor",
  "description": "Creates a trigger based on environment conditions.",
  "type": "object",
  "additionalProperties": false,
  "examples": [
    {
      "triggers": [
        {
          "event": "self:example",
          "target": "self",
          "filters": { "test": "has_tag", "value": "example" }
        }
      ]
    }
  ],
  "definitions": {
    "trigger": {
      "$ref": "../types/trigger.json"
    }
  },
  "properties": {
    "triggers": {
      "description": "The list of triggers that fire when the environment conditions match the given filter criteria.",
      "title": "Triggers",
      "oneOf": [
        {
          "type": "array",
          "items": {
            "$ref": "#/definitions/trigger"
          }
        },
        {
          "type": "object",
          "$ref": "#/definitions/trigger"
        }
      ]
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentSensor {}
