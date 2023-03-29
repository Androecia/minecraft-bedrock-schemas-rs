/* Raw contents of interact.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.interact",
  "type": "object",
  "title": "Interact",
  "description": "Defines interactions with this entity.",
  "additionalProperties": false,
  "examples": [{ "interactions": [{}] }],
  "definitions": {
    "interaction_spec": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "add_items": {
          "title": "Add Items",
          "type": "object",
          "description": "Loot table with items to add to the player's inventory upon successful interaction.",
          "additionalProperties": false,
          "properties": {
            "table": {
              "type": "string",
              "$ref": "../../../../general/loot_table/identifier.json",
              "description": "File path, relative to the Behavior Pack's path, to the loot table file.",
              "title": "Table"
            }
          }
        },
        "cooldown": {
          "title": "Cooldown",
          "type": "number",
          "default": 0,
          "description": "Time in seconds before this entity can be interacted with again."
        },
        "admire": {
          "title": "Admire",
          "type": "boolean",
          "default": false,
          "description": "Allows entity to admire the item. Requires \"minecraft:admire_item\" and \"minecraft:behavior.admire_item\" to work.",
          "$comment": "UNDOCUMENTED"
        },
        "barter": {
          "title": "Barter",
          "type": "boolean",
          "default": false,
          "description": "Allows entity to barter with the item. Requires \"minecraft:barter\" to work.",
          "$comment": "UNDOCUMENTED"
        },
        "cooldown_after_being_attacked": {
          "title": "Cooldown After Being Attacked",
          "type": "number",
          "default": 0,
          "description": "Time in seconds before this entity can be interacted with after being attacked."
        },
        "health_amount": {
          "title": "Health Amount",
          "type": "integer",
          "default": 0,
          "description": "The amount of health this entity will recover or hurt when interacting with this item. Negative values will harm the entity."
        },
        "hurt_item": {
          "title": "Hurt Item",
          "type": "integer",
          "default": 0,
          "description": "The amount of damage the item will take when used to interact with this entity. A value of 0 means the item won't lose durability."
        },
        "interact_text": {
          "title": "Interact Text",
          "type": "string",
          "default": "",
          "description": "Text to show when the player is able to interact in this way with this entity when playing with Touch-screen controls."
        },
        "on_interact": {
          "title": "On Interact",
          "$ref": "../types/trigger.json",
          "description": "Event to fire when the interaction occurs."
        },
        "particle_on_start": {
          "title": "Particle On Start",
          "type": "object",
          "description": "Particle effect that will be triggered at the start of the interaction.",
          "properties": {
            "particle_offset_towards_interactor": {
              "type": "boolean",
              "description": "Whether or not the particle will appear closer to who performed the interaction.",
              "title": "Particle Offset Towards Interactor"
            },
            "particle_type": {
              "type": "string",
              "description": "The type of particle that will be spawned.",
              "title": "Particle Type"
            },
            "particle_y_offset": {
              "type": "number",
              "description": "Will offset the particle this amount in the y direction.",
              "title": "Particle Y Offset"
            }
          }
        },
        "play_sounds": {
          "title": "Play Sounds",
          "$ref": "../../../../general/sound_event.json",
          "default": "",
          "description": "List of sounds to play when the interaction occurs."
        },
        "spawn_entities": {
          "title": "Spawn Entities",
          "type": "string",
          "default": "",
          "description": "List of entities to spawn when the interaction occurs."
        },
        "spawn_items": {
          "type": "object",
          "description": "Loot table with items to drop on the ground upon successful interaction.",
          "title": "Spawn Items",
          "additionalProperties": false,
          "properties": {
            "table": {
              "type": "string",
              "$ref": "../../../../general/loot_table/identifier.json",
              "description": "File path, relative to the Behavior Pack's path, to the loot table file.",
              "title": "Table"
            }
          }
        },
        "swing": {
          "title": "Swing",
          "type": "boolean",
          "default": false,
          "description": "If true, the player will do the \"swing\" animation when interacting with this entity."
        },
        "transform_to_item": {
          "title": "Transform To Item",
          "type": "string",
          "description": "The feed item used will transform to this item upon successful interaction. Format: itemName:auxValue"
        },
        "use_item": {
          "title": "Use Item",
          "type": "boolean",
          "default": false,
          "description": "If true, the interaction will use an item."
        },
        "vibration": {
          "title": "Vibration",
          "type": "string",
          "description": "Vibration to emit when the interaction occurs. Admitted values are entity_interact (used by default), shear, and none (no vibration emitted)."
        },
        "give_item": {
          "title": "Give Item",
          "type": "boolean",
          "$comment": "UNDOCUMENTED",
          "description": "UNDOCUMENTED Item to give to the player upon successful interaction."
        },
        "take_item": {
          "title": "Take Item",
          "type": "boolean",
          "$comment": "UNDOCUMENTED",
          "description": "UNDOCUMENTED Takes an item from the player."
        }
      }
    }
  },
  "properties": {
    "interactions": {
      "description": "The interactions.",
      "title": "Interactions",
      "oneOf": [
        {
          "type": "object",
          "$ref": "#/definitions/interaction_spec"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/definitions/interaction_spec"
          }
        }
      ]
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interact {}
