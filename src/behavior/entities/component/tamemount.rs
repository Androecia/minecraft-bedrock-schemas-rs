/* Raw contents of tamemount.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.tamemount",
  "description": "Allows the Entity to be tamed by mounting it.",
  "type": "object",
  "title": "Tamemount",
  "additionalProperties": false,
  "definitions": {
    "feed_items": {
      "type": "object",
      "description": "The list of items that can be used to increase the entity's temper and speed up the taming process.",
      "properties": {
        "item": {
          "$ref": "../../../../general/item/descriptor.json",
          "description": "Name of the item this entity likes and can be used to increase this entity's temper.",
          "title": "Item"
        },
        "temper_mod": {
          "type": "number",
          "default": 0,
          "description": "The amount of temper this entity gains when fed this item.",
          "title": "Temper Mod"
        }
      }
    },
    "auto_reject_items": {
      "type": "object",
      "description": "The list of items that this entity dislikes and will cause it to get angry if used while untamed.",
      "properties": {
        "item": {
          "$ref": "../../../../general/item/descriptor.json",
          "description": "Name of the item this entity dislikes and will cause it to get angry if used while untamed."
        }
      }
    }
  },
  "properties": {
    "attempt_temper_mod": {
      "type": "integer",
      "default": 5,
      "description": "The amount the entity's temper will increase when mounted.",
      "title": "Attempt Temper Mod"
    },
    "auto_reject_items": {
      "title": "Auto Reject Items",
      "description": "The list of items that, if carried while interacting with the entity, will anger it.",
      "oneOf": [
        {
          "type": "object",
          "$ref": "#/definitions/auto_reject_items"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/definitions/auto_reject_items"
          }
        }
      ]
    },
    "feed_text": {
      "type": "string",
      "description": "The text that shows in the feeding interact button.",
      "title": "Feed Text"
    },
    "feed_items": {
      "description": "The list of items that can be used to increase the entity's temper and speed up the taming process.",
      "title": "Feed Items",
      "oneOf": [
        {
          "type": "object",
          "$ref": "#/definitions/feed_items"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/definitions/feed_items"
          }
        }
      ]
    },
    "max_temper": {
      "type": "integer",
      "default": 100,
      "description": "The maximum value for the entity's random starting temper.",
      "title": "Maximum Temper"
    },
    "min_temper": {
      "type": "integer",
      "default": 0,
      "description": "The minimum value for the entity's random starting temper.",
      "title": "Minimum Temper"
    },
    "ride_text": {
      "type": "string",
      "description": "The text that shows in the riding interact button.",
      "title": "Ride Text"
    },
    "tame_event": {
      "$ref": "../types/event_object.json",
      "description": "Event that triggers when the entity becomes tamed.",
      "title": "Tame Event"
    }
  },
  "examples": [
    {
      "attempt_temper_mod": 5,
      "feed_text": "example",
      "max_temper": 100,
      "min_temper": 0,
      "ride_text": "example"
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tamemount {}
