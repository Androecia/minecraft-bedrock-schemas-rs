/* Raw contents of spawn_entity.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.spawn_entity",
  "type": "object",
  "description": "Adds a timer after which this entity will spawn another entity or item (similar to vanilla's chicken's egg-laying behavior).",
  "title": "Spawn Entity",
  "additionalProperties": false,
  "definitions": {
    "entity_spawn": {
      "additionalProperties": false,
      "type": "object",
      "title": "Entity Spawn",
      "properties": {
        "filters": {
          "description": "If present, the specified entity will only spawn if the filter evaluates to true.",
          "$ref": "../../filters/filters.json"
        },
        "max_wait_time": {
          "type": "integer",
          "default": 600,
          "description": "Maximum amount of time to randomly wait in seconds before another entity is spawned.",
          "title": "Maximum Wait Time"
        },
        "min_wait_time": {
          "type": "integer",
          "default": 300,
          "description": "Minimum amount of time to randomly wait in seconds before another entity is spawned.",
          "title": "Minimum Wait Time"
        },
        "num_to_spawn": {
          "type": "integer",
          "default": 1,
          "description": "The number of entities of this type to spawn each time that this triggers.",
          "title": "Num To Spawn"
        },
        "should_leash": {
          "type": "boolean",
          "default": false,
          "description": "If true, this the spawned entity will be leashed to the parent.",
          "title": "Should Leash"
        },
        "single_use": {
          "type": "boolean",
          "default": false,
          "description": "If true, this component will only ever spawn the specified entity once.",
          "title": "Single Use"
        },
        "spawn_entity": {
          "type": "string",
          "default": "",
          "description": "Identifier of the entity to spawn, leave empty to spawn the item defined above instead.",
          "title": "Spawn Entity"
        },
        "spawn_event": {
          "type": "string",
          "default": "minecraft:entity_born",
          "description": "Event to call when the entity is spawned.",
          "title": "Spawn Event"
        },
        "spawn_item": {
          "$ref": "../../../../general/item/descriptor.json",
          "default": "egg",
          "description": "Item identifier of the item to spawn.",
          "title": "Spawn Item"
        },
        "spawn_method": {
          "type": "string",
          "default": "born",
          "description": "Method to use to spawn the entity.",
          "title": "Spawn Method"
        },
        "spawn_sound": {
          "$ref": "../../../../general/sound_event.json",
          "default": "plop",
          "description": "Identifier of the sound effect to play when the entity is spawned.",
          "title": "Spawn Sound"
        }
      }
    }
  },
  "properties": {
    "entities": {
      "title": "Entities",
      "description": "The entities to spawn.",
      "oneOf": [
        {
          "type": "object",
          "$ref": "#/definitions/entity_spawn"
        },
        {
          "type": "array",
          "items": {
            "type": "object",
            "$ref": "#/definitions/entity_spawn"
          }
        }
      ]
    }
  }
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntity {}
