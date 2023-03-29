/* Raw contents of block_sensor.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.block_sensor",
  "type": "object",
  "title": "Block Sensor",
  "description": "Fires off a specified event when a block in the block list is broken within the sensor range.",
  "additionalProperties": false,
  "properties": {
    "sensor_radius": {
      "title": "Sensor Radius",
      "type": "integer",
      "description": "The maximum radial distance in which a specified block can be detected. The biggest radius is 32.0.",
      "minimum": 0,
      "maximum": 32
    },
    "on_break": {
      "title": "On Break",
      "type": "array",
      "description": "Blocks that will trigger the component when broken and what event will trigger.",
      "items": {
        "title": "On Block Broken",
        "type": "object",
        "description": "Event to run when a block breaks.",
        "additionalProperties": false,
        "properties": {
          "block_list": {
            "title": "Block List",
            "type": "array",
            "description": "List of blocks that will trigger the sensor.",
            "items": {
              "$ref": "../../../../general/block/identifier.json",
              "title": "Block ID"
            }
          },
          "on_block_broken": {
            "title": "On Block Broken",
            "type": "string",
            "description": "Event to run when a block breaks."
          }
        }
      }
    },
    "sources": {
      "title": "Sources",
      "description": "List of sources that break the block to listen for. If none are specified, all block breaks will be detected.",
      "type": "array",
      "items": {
        "$ref": "../../filters/filters.json"
      },
      "examples": [
        {
          "test": "has_silk_touch",
          "subject": "other",
          "value": false
        }
      ]
    }
  },
  "examples": [
    {
      "sensor_radius": 0,
      "on_break": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockSensor {}
