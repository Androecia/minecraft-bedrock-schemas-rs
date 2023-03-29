/* Raw contents of entity_sensor.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.entity_sensor",
  "type": "object",
  "title": "Entity Sensor",
  "description": "A component that fires an event when a set of conditions are met by other entities within the defined range.",
  "additionalProperties": false,
  "properties": {
    "maximum_count": {
      "type": "integer",
      "default": -1,
      "description": "The maximum number of entities that must pass the filter conditions for the event to send.",
      "title": "Maximum Count"
    },
    "minimum_count": {
      "type": "integer",
      "default": 1,
      "description": "The minimum number of entities that must pass the filter conditions for the event to send.",
      "title": "Minimum Count"
    },
    "relative_range": {
      "type": "boolean",
      "default": true,
      "description": "If true the sensor range is additive on top of the entity's size.",
      "title": "Relative Range"
    },
    "require_all": {
      "type": "boolean",
      "default": false,
      "description": "If true requires all nearby entities to pass the filter conditions for the event to send.",
      "title": "Require All"
    },
    "sensor_range": {
      "type": "number",
      "default": 10,
      "description": "The maximum distance another entity can be from this and have the filters checked against it.",
      "title": "Sensor Range"
    },
    "event_filters": {
      "$ref": "../../filters/filters.json"
    },
    "event": {
      "title": "Event",
      "description": "event.",
      "type": "string"
    }
  },
  "examples": [
    {
      "maximum_count": -1,
      "minimum_count": 1,
      "relative_range": true,
      "require_all": false,
      "sensor_range": 10,
      "event": "example"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntitySensor {}
