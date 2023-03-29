/* Raw contents of scale_by_age.json That I want to use to make a rust Struct from this Json Schema:
{
  "$id": "blockception.minecraft.behavior.entities.minecraft.scale_by_age",
  "type": "object",
  "title": "Scale By Age",
  "additionalProperties": false,
  "description": "Defines the entity's size interpolation based on the entity's age.",
  "required": [],
  "properties": {
    "end_scale": {
      "type": "number",
      "default": 1,
      "description": "Ending scale of the entity when it's fully grown.",
      "title": "End Scale"
    },
    "start_scale": {
      "type": "number",
      "default": 1,
      "description": "Initial scale of the newborn entity.",
      "title": "Start Scale"
    }
  },
  "examples": [
    {
      "end_scale": 1,
      "start_scale": 1
    }
  ]
}
*/ use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScaleByAge {}
