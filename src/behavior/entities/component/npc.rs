/* Raw contents of npc.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.npc",
  "type": "object",
  "title": "Npc",
  "description": "Sets this entity as an NPC",
  "$comment": "UNDOCUMENTED",
  "additionalProperties": false,
  "definitions": {
    "rangeXYZ": {
      "type": "array",
      "items": [
        {
          "type": "number",
          "title": "X"
        },
        {
          "type": "number",
          "title": "Y"
        },
        {
          "type": "number",
          "title": "Z"
        }
      ]
    }
  },
  "properties": {
    "npc_data": {
      "type": "object",
      "title": "Npc Data",
      "description": "The data belonging to this npc.",
      "$comment": "UNDOCUMENTED",
      "additionalProperties": false,
      "properties": {
        "portrait_offsets": {
          "type": "object",
          "title": "Portrait Offsets",
          "description": "UNDOCUMENTED.",
          "$comment": "UNDOCUMENTED",
          "additionalProperties": false,
          "properties": {
            "translate": {
              "$ref": "#/definitions/rangeXYZ",
              "title": "Translate",
              "description": "UNDOCUMENTED.",
              "$comment": "UNDOCUMENTED"
            },
            "scale": {
              "$ref": "#/definitions/rangeXYZ",
              "title": "Scale",
              "description": "UNDOCUMENTED.",
              "$comment": "UNDOCUMENTED"
            }
          }
        },
        "picker_offsets": {
          "type": "object",
          "title": "Picker Offsets",
          "description": "UNDOCUMENTED.",
          "$comment": "UNDOCUMENTED",
          "additionalProperties": false,
          "properties": {
            "translate": {
              "$ref": "#/definitions/rangeXYZ",
              "title": "Translate",
              "description": "UNDOCUMENTED.",
              "$comment": "UNDOCUMENTED"
            },
            "scale": {
              "$ref": "#/definitions/rangeXYZ",
              "title": "Scale",
              "description": "UNDOCUMENTED.",
              "$comment": "UNDOCUMENTED"
            }
          }
        },
        "skin_list": {
          "type": "array",
          "title": "Skin List",
          "description": "UNDOCUMENTED.",
          "$comment": "UNDOCUMENTED",
          "items": {
            "type": "object",
            "title": "Skin",
            "description": "UNDOCUMENTED.",
            "$comment": "UNDOCUMENTED",
            "additionalProperties": false,
            "properties": {
              "variant": {
                "title": "Variant",
                "description": "UNDOCUMENTED.",
                "$comment": "UNDOCUMENTED",
                "type": "integer",
                "minimum": 0
              },
              "mark_variant": {
                "title": "Mark Variant",
                "description": "UNDOCUMENTED.",
                "$comment": "UNDOCUMENTED",
                "type": "integer",
                "minimum": 0
              }
            }
          }
        }
      }
    }
  },
  "examples": [
    {
      "npc_data": {}
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Npc {}
