/* Raw contents of rideable.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.rideable",
  "description": "Determines whether this entity can be ridden. Allows specifying the different seat positions and quantity.",
  "type": "object",
  "title": "Rideable",
  "additionalProperties": false,
  "definitions": {
    "seats_spec": {
      "additionalProperties": false,
      "type": "object",
      "properties": {
        "lock_rider_rotation": {
          "type": "number",
          "default": 181,
          "description": "Angle in degrees that a rider is allowed to rotate while riding this entity. Omit this property for no limit"
        },
        "max_rider_count": {
          "type": "integer",
          "default": 0,
          "description": "Defines the maximum number of riders that can be riding this entity for this seat to be valid."
        },
        "min_rider_count": {
          "type": "integer",
          "default": 0,
          "description": "Defines the minimum number of riders that need to be riding this entity before this seat can be used."
        },
        "position": {
          "$ref": "../../../../general/vectors/number3.json",
          "default": [0.0, 0.0, 0.0],
          "description": "Position of this seat relative to this entity's position."
        },
        "rotate_rider_by": {
          "$ref": "../../../../molang/number.json",
          "description": "Offset to rotate riders by."
        }
      }
    }
  },
  "properties": {
    "controlling_seat": {
      "type": "integer",
      "default": 0,
      "description": "The seat that designates the driver of the entity.",
      "title": "Controlling Seat"
    },
    "crouching_skip_interact": {
      "type": "boolean",
      "default": true,
      "description": "If true, this entity can't be interacted with if the entity interacting with it is crouching.",
      "title": "Crouching Skip Interact"
    },
    "family_types": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "description": "List of entities that can ride this entity.",
      "title": "Family Types"
    },
    "interact_text": {
      "type": "string",
      "default": "",
      "description": "The text to display when the player can interact with the entity when playing with Touch-screen controls.",
      "title": "Interact Text"
    },
    "pull_in_entities": {
      "type": "boolean",
      "default": false,
      "description": "If true, this entity will pull in entities that are in the correct family_types into any available seats.",
      "title": "Pull In Entities"
    },
    "rider_can_interact": {
      "type": "boolean",
      "default": false,
      "description": "If true, this entity will be picked when looked at by the rider.",
      "title": "Rider Can Interact"
    },
    "seat_count": {
      "type": "integer",
      "default": 1,
      "description": "The number of entities that can ride this entity at the same time.",
      "title": "Seat Count"
    },
    "seats": {
      "description": "The list of positions and number of riders for each position for entities riding this entity.",
      "title": "Seats",
      "oneOf": [
        {
          "type": "object",
          "$ref": "#/definitions/seats_spec"
        },
        {
          "type": "array",
          "items": {
            "$ref": "#/definitions/seats_spec"
          }
        }
      ]
    }
  },
  "examples": [
    {
      "controlling_seat": 0,
      "crouching_skip_interact": true,
      "family_types": [],
      "interact_text": "",
      "pull_in_entities": false,
      "rider_can_interact": false,
      "seat_count": 1
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rideable {}
