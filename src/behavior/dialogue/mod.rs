/*{
    "$schema": "http://json-schema.org/draft-07/schema",
    "$id": "blockception.minecraft.behavior.dialogue",
    "examples": [
        {
            "format_version": "1.19.0",
            "minecraft:npc_dialogue": {
                "scenes": [
                    {
                        "scene_tag": "fast_travel",
                        "npc_name": { "rawtext": [{ "translate": "dialogue.guide.name" }] },
                        "text": { "rawtext": [{ "translate": "dialogue.fast_travel.body", "with": ["\n"] }] },
                        "buttons": []
                    }
                ]
            }
        }
    ],
    "type": "object",
    "title": "NPC Dialogue",
    "description": "Specifies the dialogue scenes.",
    "$comment": "UNDOCUMENTED",
    "additionalProperties": false,
    "required": ["format_version", "minecraft:npc_dialogue"],
    "properties": {
        "format_version": { "$ref": "../../general/format_version.json" },
        "minecraft:npc_dialogue": {
            "title": "NPC Dialogue",
            "description": "Specifies the dialogue of an npc.",
            "$comment": "UNDOCUMENTED",
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "scenes": {
                    "title": "Scenes",
                    "description": "The different scenes.",
                    "$comment": "UNDOCUMENTED",
                    "type": "array",
                    "minItems": 1,
                    "items": {
                        "title": "Scene",
                        "description": "A single scene specification.",
                        "$comment": "UNDOCUMENTED",
                        "type": "object",
                        "additionalProperties": false,
                        "required": ["scene_tag"],
                        "examples": [{ "scene_tag": "foo_intro" }],
                        "properties": {
                            "buttons": {
                                "title": "Buttons",
                                "description": "This is where you can create buttons for your NPC.",
                                "type": "array",
                                "maxItems": 6,
                                "items": {
                                    "title": "Button",
                                    "description": "This is where you can create buttons for your NPC.",
                                    "type": "object",
                                    "additionalProperties": false,
                                    "properties": {
                                        "name": {
                                            "title": "Name",
                                            "description": "Set the text that is going to be displayed on your NPC’s button.",
                                            "oneOf": [{ "type": "string" }, { "$ref": "../../general/rawtext/rawtext.json" }]
                                        },
                                        "commands": {
                                            "type": "array",
                                            "description": "allows you to add commands which will be run in-game when the button is pressed.",
                                            "title": "Commands",
                                            "items": {
                                                "title": "Minecraft Command",
                                                "description": "The commands to execute.",
                                                "type": "string",
                                                "pattern": "^/[a-z].*$"
                                            }
                                        }
                                    }
                                }
                            },
                            "npc_name": {
                                "title": "NPC Name",
                                "description": "This is where you can add or change a name for your NPC dialogue box. This is an optional property that is useful for dynamically changing NPC names.",
                                "oneOf": [{ "type": "string" }, { "$ref": "../../general/rawtext/rawtext.json" }]
                            },
                            "on_close_commands": {
                                "type": "array",
                                "description": "This is where you can define which commands will fire when the NPC dialogue box closes.",
                                "title": "On Close Commands",
                                "items": {
                                    "title": "Minecraft Command",
                                    "description": "A minecraft command to execute.",
                                    "type": "string",
                                    "pattern": "^/[a-z].*$"
                                }
                            },
                            "on_open_commands": {
                                "type": "array",
                                "description": "This is where you can define which commands will fire when the NPC dialogue box opens.",
                                "title": "On Close Commands",
                                "items": {
                                    "title": "Minecraft Command",
                                    "description": "A minecraft command to execute.",
                                    "type": "string",
                                    "pattern": "^/[a-z].*$"
                                }
                            },
                            "scene_tag": {
                                "title": "Scene Tag",
                                "description": "This is the name you will use to call this scene in-game. This is a required property.",
                                "type": "string"
                            },
                            "text": {
                                "title": "Text",
                                "description": "This is where you enter the dialogue you want your NPC to display in-game for this scene. You can type the dialogue text directly here or use raw text if you are using a language file. This is an optional property, but without it your NPC dialogue box will be empty.",
                                "oneOf": [{ "type": "string" }, { "$ref": "../../general/rawtext/rawtext.json" }]
                            }
                        }
                    }
                }
            }
        }
    }
}
 */

use crate::general::version::Version;

use serde::{Deserialize, Serialize};

/// Specifies the dialogue of an npc.

#[derive(Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub format_version: Version,

    /// The different scenes.
    #[serde(rename = "minecraft:npc_dialogue")]
    pub npc_dialogue: NpcDialogue,
}

#[derive(Clone, Serialize, Deserialize)]
/// Specifies the dialogue of an npc.
pub struct NpcDialogue {
    /// The different scenes.
    pub scenes: Vec<Scene>,
}

#[derive(Serialize, Deserialize, Clone)]
/// A single scene specification.
pub struct Scene {
    /// This is the name you will use to call this scene in-game. This is a required property.
    pub scene_tag: String,

    /// This is where you can create buttons for your NPC.
    pub buttons: Option<Vec<Button>>,

    /// This is where you can add or change a name for your NPC dialogue box. This is an optional property that is useful for dynamically changing NPC names.
    pub npc_name: Option<String>,

    /// This is where you can define which commands will fire when the NPC dialogue box closes.
    pub on_close_commands: Option<Vec<SlashCommand>>,

    /// This is where you can define which commands will fire when the NPC dialogue box opens.
    pub on_open_commands: Option<Vec<SlashCommand>>,

    /// This is where you enter the dialogue you want your NPC to display in-game for this scene. You can type the dialogue text directly here or use raw text if you are using a language file. This is an optional property, but without it your NPC dialogue box will be empty.
    pub text: Option<String>,
}

use crate::slash_command::SlashCommand;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// This is where you can create buttons for your NPC.
pub struct Button {
    /// Set the text that is going to be displayed on your NPC’s button.
    pub name: String,

    /// allows you to add commands which will be run in-game when the button is pressed.
    pub commands: Vec<SlashCommand>,
}

// impl functions which is a vec of commands but if one command fails the rest will not be executed
// maybe command can be a trait which other commands can implement

use crate::general::rawtext::RawText;

pub enum Text {
    RawText(RawText),
    String(String),
}

//#[test]
fn deserialize() {
    let paths = vec![
        "./bedrock-samples/resource_pack/fogs/fog.json".to_string(),
        "../Minecraft-bedrock-json-schemas/test/files/correct/data_rp/fogs/fog.json".to_string(),
    ];

    //crate::utils::test_serde_from_files_in_path::<Dialogue>(paths);
}
