/* Raw contents of conditional_bandwidth_optimization.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.conditional_bandwidth_optimization",
  "additionalProperties": false,
  "type": "object",
  "title": "Conditional Bandwidth Optimization",
  "description": "Defines the Conditional Spatial Update Bandwidth Optimizations of this entity.",
  "properties": {
    "conditional_values": {
      "title": "Conditional Values",
      "description": "The object containing the conditional bandwidth optimization values.",
      "type": "array",
      "items": {
        "title": "Conditional Value",
        "description": "The object containing the conditional bandwidth optimization values.",
        "additionalProperties": false,
        "type": "object",
        "properties": {
          "max_dropped_ticks": {
            "title": "Maximum Dropped Ticks",
            "description": "In relation to the optimization value, determines the maximum ticks spatial update packets can be not sent.",
            "type": "integer"
          },
          "max_optimized_distance": {
            "title": "Maximum Optimized Distance",
            "description": "The maximum distance considered during bandwidth optimizations. Any value below the Maximum is interpolated to find optimization, and any value greater than or equal to this Maximum results in Maximum optimization.",
            "type": "number"
          },
          "use_motion_prediction_hints": {
            "title": "Use Motion Prediction Hints",
            "description": "When set to true, smaller motion packets will be sent during drop packet intervals, resulting in the same amount of packets being sent as without optimizations but with much less data being sent. This should be used when actors are travelling very quickly or teleporting to prevent visual oddities.",
            "type": "boolean"
          },
          "conditional_values": {
            "title": "Conditional Values",
            "description": "Conditions that must be met for these optimization values to be used.",
            "type": "array",
            "items": {
              "$ref": "../../filters/filters.json"
            }
          }
        }
      }
    },
    "default_values": {
      "title": "Default Values",
      "description": "The object containing the default bandwidth optimization values.",
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "max_dropped_ticks": {
          "title": "Maximum Dropped Ticks",
          "description": "In relation to the optimization value, determines the maximum ticks spatial update packets can be not sent.",
          "type": "integer"
        },
        "max_optimized_distance": {
          "title": "Maximum Optimized Distance",
          "description": "The maximum distance considered during bandwidth optimizations. Any value below the Maximum is interpolated to find optimization, and any value greater than or equal to this Maximum results in Maximum optimization.",
          "type": "number"
        },
        "use_motion_prediction_hints": {
          "title": "Use Motion Prediction Hints",
          "description": "When set to true, smaller motion packets will be sent during drop packet intervals, resulting in the same amount of packets being sent as without optimizations but with much less data being sent. This should be used when actors are travelling very quickly or teleporting to prevent visual oddities.",
          "type": "boolean"
        }
      }
    }
  },
  "examples": [
    {
      "conditional_values": [],
      "default_values": {}
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConditionalBandwidthOptimization {}
