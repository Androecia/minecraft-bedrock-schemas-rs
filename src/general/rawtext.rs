/*
The Json Schema to convert to Rust code is:
{
  "title": "RawText",
  "description": "A json structure that allows for translations, or go from scores and selectors to text.",
  "$comment": "UNDOCUMENTED",
  "type": "object",
  "additionalProperties": false,
  "required": ["rawtext"],
  "examples": [{ "rawtext": [] }],
  "definitions": {
    "rawtext": {
      "title": "Rawtext",
      "description": "The raw text component, which consists of an array of text components.",
      "$comment": "UNDOCUMENTED",
      "type": "array",
      "default": [],
      "example": [[{ "translate": "example.language.key" }]],
      "items": {
        "oneOf": [{ "type": "string" }, { "$ref": "#/definitions/translate" }, { "$ref": "#/definitions/text" }, { "$ref": "#/definitions/selector" }, { "$ref": "#/definitions/score" }]
      }
    },
    "selector": {
      "title": "Selector",
      "description": "A text component that turns a selector into text, will usually display like: `Player1, Player2 and Player3`.",
      "$comment": "UNDOCUMENTED",
      "type": "object",
      "additionalProperties": false,
      "required": ["selector"],
      "examples": [{ "selector": "@s" }, { "selector": "@p" }],
      "properties": {
        "selector": {
          "title": "Selector",
          "description": "The selector to target, for dialogue files, you can use @initiator.",
          "$comment": "UNDOCUMENTED",
          "type": "string",
          "examples": ["@a", "@s", "@r", "@p", "@e", "@initiator"],
          "pattern": "^@.*$"
        }
      }
    },
    "score": {
      "title": "Score",
      "description": "A text component that grabs the score from an given target and turns its value of a specified score.",
      "$comment": "UNDOCUMENTED",
      "type": "object",
      "additionalProperties": false,
      "required": ["score"],
      "examples": [{ "score": { "name": "*", "objective": "score" } }, { "score": { "name": "@p", "objective": "score" } }],
      "properties": {
        "score": {
          "title": "Score",
          "description": "The score text component.",
          "$comment": "UNDOCUMENTED",
          "type": "object",
          "additionalProperties": false,
          "required": ["name", "objective"],
          "examples": [
            { "name": "*", "objective": "random" },
            { "name": "@a[score={count=0..},c=1]", "objective": "count" }
          ],
          "properties": {
            "name": {
              "title": "Selector",
              "description": "A selector, player name (can be fake), or * to target who is reading the message.",
              "type": "string",
              "examples": ["@a", "@s", "@r", "@p", "@e", "@initiator", "*"]
            },
            "objective": {
              "title": "Objective",
              "description": "The scoreboard objective to retrieve the value of.",
              "$comment": "UNDOCUMENTED",
              "type": "string"
            }
          }
        }
      }
    },
    "text": {
      "title": "Text",
      "description": "A simple text component, will display the text raw (without processing).",
      "$comment": "UNDOCUMENTED",
      "type": "object",
      "additionalProperties": false,
      "required": ["text"],
      "examples": [{ "text": "Hello World!" }],
      "properties": {
        "text": {
          "title": "Text",
          "description": "The text to display.",
          "$comment": "UNDOCUMENTED",
          "type": "string"
        }
      }
    },
    "translate": {
      "title": "Translate",
      "description": "A text component that will attempt to translate the given key through the languages files.",
      "$comment": "UNDOCUMENTED",
      "type": "object",
      "additionalProperties": false,
      "required": ["translate"],
      "examples": [{ "translate": "example.translation.key" }, { "translate": "example.translation.key", "with": ["\n"] }],
      "properties": {
        "translate": {
          "title": "Translate",
          "description": "The key to translate.",
          "$comment": "UNDOCUMENTED",
          "type": "string",
          "examples": ["example.translation.key"]
        },
        "with": {
          "$ref": "#/definitions/with"
        }
      }
    },
    "with": {
      "title": "With",
      "description": "Specifies for the translator that additional text component needs to be inserted, this will cause the translator to look for variables in the translation text and replaced them with the corresponding 'With' text component.",
      "$comment": "UNDOCUMENTED",
      "examples": ["\n", { "rawtext": [] }],
      "items": {
        "oneOf": [
          { "type": "string" },
          {
            "title": "Rawtext",
            "description": "Specifies that this 'with' component needs to be processed.",
            "$comment": "UNDOCUMENTED",
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "rawtext": {
                "$ref": "#/definitions/rawtext"
              }
            }
          }
        ]
      }
    }
  },
  "properties": {
    "rawtext": {
      "$ref": "#/definitions/rawtext"
    }
  }
}
 */

use crate::general::selection::{WildcardSelection,Selection};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct RawText {
    // The raw text component, which consists of an array of text components.
    pub rawtext: Vec<TextComponent>,
}

impl RawText {
    pub fn new(rawtext: Vec<TextComponent>) -> Self {
        Self {
            rawtext,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RawTextComponent {
    /// A text component that will attempt to translate the given key through the languages files.
    Translation(TranslationComponent),
    /// A simple text component, will display the text raw (without processing).
    Text(TextComponent),
    /// A text component that turns a selector into text, will usually display like: `Player1, Player2 and Player3`.
    Selector(Selection),
    /// A text component that grabs the score from an given target and turns its value of a specified score.
    Score(ScoreComponent),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A text component that will attempt to translate the given key through the languages files.
pub struct TranslationComponent {
    // The key to translate.
    pub translate: String,
    // Specifies for the translator that additional text component needs to be inserted, this will cause the translator to look for variables in the translation text and replaced them with the corresponding 'With' text component.
    pub with: Option<Vec<With>>,
}

impl TranslationComponent {
    pub fn new(translate: String, with: Option<Vec<With>>) -> Self {
        Self {
            translate,
            with,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum With {
    String(String),
    RawText(RawText),
}







#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextComponent {
    // The text to display.
    pub text: String,
}

impl TextComponent {
    pub fn new(text: String) -> Self {
        Self {
            text,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct ScoreComponent {
    // A selector , player name (can be fake), or * to target who is reading the message.
    pub name: WildcardSelection,
    // The scoreboard objective to retrieve the value of.
    pub objective: String,
}













