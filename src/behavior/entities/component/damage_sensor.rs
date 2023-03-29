/* Raw contents of damage_sensor.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.damage_sensor",
  "title": "Damage Sensor",
  "description": "Defines what events to call when this entity is damaged by specific entities or items.",
  "type": "object",
  "additionalProperties": false,
  "examples": [
    {
      "triggers": [
        {
          "cause": "all",
          "deals_damage": false
        }
      ]
    },
    {
      "triggers": [
        {
          "on_damage": { "filters": { "test": "has_damage", "subject": "self", "value": "fatal" } },
          "deals_damage": false
        }
      ]
    }
  ],
  "definitions": {
    "triggers": {
      "title": "Triggers",
      "description": "List of triggers with the events to call when taking specific kinds of damage.",
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "cause": {
          "type": "string",
          "default": "none",
          "description": "Type of damage that triggers the events.",
          "title": "Cause",
          "$ref": "../../../../general/entity/damage_source.json"
        },
        "damage_modifier": {
          "title": "Damage Modifier",
          "type": "number",
          "default": 0.0,
          "description": "A modifier that adds to/removes from the base damage from the damage cause. It does not reduce damage to less than 0."
        },
        "damage_multiplier": {
          "type": "number",
          "default": 1,
          "description": "A multiplier that modifies the base damage from the damage cause. If deals_damage is true the multiplier can only reduce the damage the entity will take to a minimum of 1.",
          "title": "Damage Multiplier"
        },
        "deals_damage": {
          "type": "boolean",
          "default": true,
          "description": "If true, the damage dealt to the entity will take away health from it, set to false to make the entity ignore that damage.",
          "title": "Deals Damage"
        },
        "on_damage": {
          "type": "object",
          "description": "Specifies filters for entity definitions and events.",
          "title": "On Damage",
          "$ref": "../types/trigger.json"
        },
        "on_damage_sound_event": {
          "$ref": "../../../../general/sound_event.json",
          "description": "Defines what sound to play, if any, when the on_damage filters are met.",
          "title": "On Damage Sound Event"
        }
      }
    }
  },
  "properties": {
    "triggers": {
      "title": "Triggers",
      "description": "The list of triggers that fire when the environment conditions match the given filter criteria.",
      "examples": [
        [
          {
            "cause": "all",
            "deals_damage": false
          }
        ]
      ],
      "oneOf": [
        {
          "type": "array",
          "items": {
            "$ref": "#/definitions/triggers"
          }
        },
        {
          "type": "object",
          "$ref": "#/definitions/triggers"
        }
      ]
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageSensor {}
