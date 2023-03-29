/* Raw contents of spell_effects.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.spell_effects",
  "type": "object",
  "title": "Spell Effects",
  "additionalProperties": false,
  "description": "Defines what mob effects to add and remove to the entity when adding this component.",
  "required": [],
  "properties": {
    "add_effects": {
      "type": "array",
      "description": "List of effects to add to this entity after adding this component.",
      "items": {
        "oneOf": [
          { "type": "string" },
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "amplifier": {
                "type": "integer",
                "title": "Amplifier",
                "description": "The level of the effect, same as used in the /effect command (0 for level I, 1 for level II, etc). Defaults to 0. NOTE: Values can be negative but its not an intentional feature",
                "default": 0,
                "minimum": 0
              },
              "ambient": {
                "title": "Ambient",
                "description": "Boolean value that should cause the particles emitted by the entity to be partially transparent. This does not work properly, resulting in this property having no effect. Defaults to false.",
                "default": false,
                "type": "boolean"
              },
              "duration": {
                "title": "Duration",
                "description": "The amount of time in seconds the effect should last. This allows for fractional numbers. For example, instant effects should be set to 0.05 seconds (one tick).",
                "type": "number",
                "minimum": 0
              },
              "display_on_screen_animation": {
                "type": "boolean",
                "title": "Display On Screen Animation",
                "description": "Boolean value. When set to true, applying this effect displays an animated graphic on-screen similar to the totem of undying effect. Obviously, this only works for players. Defaults to false."
              },
              "effect": {
                "type": "string",
                "title": "Effect",
                "description": "The string identifier of the status effect to add. These are the same as used in the /effect command."
              },
              "visible": {
                "type": "boolean",
                "title": "Visible",
                "description": "Boolean value. When set to true, the effect will be visible to the player. Defaults to true."
              }
            }
          }
        ]
      },
      "title": "Add Effects"
    },
    "remove_effects": {
      "title": "Remove Effects",
      "description": "List of identifiers of effects to be removed from this entity after adding this component.",
      "oneOf": [
        {
          "type": "array",
          "items": {
            "type": ["string"],
            "title": "Spell Effect ID",
            "description": "identifier of the effect to be removed from this entity after adding this component."
          }
        },
        {
          "type": "string"
        }
      ]
    }
  },
  "examples": [
    {
      "add_effects": []
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpellEffects {}
