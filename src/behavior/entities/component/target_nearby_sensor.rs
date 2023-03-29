/* Raw contents of target_nearby_sensor.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.target_nearby_sensor",
  "description": "Defines the entity's range within which it can see or sense other entities to target them.",
  "type": "object",
  "title": "Target Nearby Sensor",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "must_see": {
      "type": "boolean",
      "default": false,
      "description": "Whether the other entity needs to be visible to trigger `inside` events.",
      "title": "Must See"
    },
    "inside_range": {
      "type": "number",
      "default": 1,
      "description": "Maximum distance in blocks that another entity will be considered in the `inside` range.",
      "title": "Inside Range"
    },
    "on_inside_range": {
      "$ref": "../types/event_object.json",
      "description": "Event to call when an entity gets in the inside range. Can specify `event` for the name of the event and `target` for the target of the event",
      "title": "On Inside Range"
    },
    "on_outside_range": {
      "$ref": "../types/event_object.json",
      "description": "Event to call when an entity gets in the outside range. Can specify `event` for the name of the event and `target` for the target of the event",
      "title": "On Outside Range"
    },
    "on_vision_lost_inside_range": {
      "$ref": "../types/event_object.json",
      "description": "Event to call when an entity exits visual range. Can specify `event` for the name of the event and `target` for the target of the event",
      "title": "On Vision Lost Inside Range"
    },
    "outside_range": {
      "type": "number",
      "default": 5,
      "description": "Maximum distance in blocks that another entity will be considered in the `outside` range.",
      "title": "Outside Range"
    }
  },
  "examples": [
    {
      "must_see": false,
      "inside_range": 1,
      "outside_range": 5
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetNearbySensor {}
