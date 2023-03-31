
use schemars::schema::{RootSchema,Schema};

use serde::{Serialize, Deserialize};

use serde_json;


#[derive(Serialize, Deserialize,Debug)]
struct CompressSpec {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "Files")]
    files: Vec<CompressSpecFile>


}
#[derive(Serialize, Deserialize,Debug)]
struct CompressSpecFile {
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "Destination")]
    destination: String
}

use std::fs;
use std::path::Path;

use jsonschema::{JSONSchema, Draft};





const DRAFT_7_META_SCHEMA: &str = r##"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "http://json-schema.org/draft-07/schema#",
    "title": "Core schema meta-schema",
    "definitions": {
        "schemaArray": {
            "type": "array",
            "minItems": 1,
            "items": { "$ref": "#" }
        },
        "nonNegativeInteger": {
            "type": "integer",
            "minimum": 0
        },
        "nonNegativeIntegerDefault0": {
            "allOf": [
                { "$ref": "#/definitions/nonNegativeInteger" },
                { "default": 0 }
            ]
        },
        "simpleTypes": {
            "enum": [
                "array",
                "boolean",
                "integer",
                "null",
                "number",
                "object",
                "string"
            ]
        },
        "stringArray": {
            "type": "array",
            "items": { "type": "string" },
            "uniqueItems": true,
            "default": []
        }
    },
    "type": ["object", "boolean"],
    "properties": {
        "$id": {
            "type": "string",
            "format": "uri-reference"
        },
        "$schema": {
            "type": "string",
            "format": "uri"
        },
        "$ref": {
            "type": "string",
            "format": "uri-reference"
        },
        "$comment": {
            "type": "string"
        },
        "title": {
            "type": "string"
        },
        "description": {
            "type": "string"
        },
        "default": true,
        "readOnly": {
            "type": "boolean",
            "default": false
        },
        "writeOnly": {
            "type": "boolean",
            "default": false
        },
        "examples": {
            "type": "array",
            "items": true
        },
        "multipleOf": {
            "type": "number",
            "exclusiveMinimum": 0
        },
        "maximum": {
            "type": "number"
        },
        "exclusiveMaximum": {
            "type": "number"
        },
        "minimum": {
            "type": "number"
        },
        "exclusiveMinimum": {
            "type": "number"
        },
        "maxLength": { "$ref": "#/definitions/nonNegativeInteger" },
        "minLength": { "$ref": "#/definitions/nonNegativeIntegerDefault0" },
        "pattern": {
            "type": "string",
            "format": "regex"
        },
        "additionalItems": { "$ref": "#" },
        "items": {
            "anyOf": [
                { "$ref": "#" },
                { "$ref": "#/definitions/schemaArray" }
            ],
            "default": true
        },
        "maxItems": { "$ref": "#/definitions/nonNegativeInteger" },
        "minItems": { "$ref": "#/definitions/nonNegativeIntegerDefault0" },
        "uniqueItems": {
            "type": "boolean",
            "default": false
        },
        "contains": { "$ref": "#" },
        "maxProperties": { "$ref": "#/definitions/nonNegativeInteger" },
        "minProperties": { "$ref": "#/definitions/nonNegativeIntegerDefault0" },
        "required": { "$ref": "#/definitions/stringArray" },
        "additionalProperties": { "$ref": "#" },
        "definitions": {
            "type": "object",
            "additionalProperties": { "$ref": "#" },
            "default": {}
        },
        "properties": {
            "type": "object",
            "additionalProperties": { "$ref": "#" },
            "default": {}
        },
        "patternProperties": {
            "type": "object",
            "additionalProperties": { "$ref": "#" },
            "propertyNames": { "format": "regex" },
            "default": {}
        },
        "dependencies": {
            "type": "object",
            "additionalProperties": {
                "anyOf": [
                    { "$ref": "#" },
                    { "$ref": "#/definitions/stringArray" }
                ]
            }
        },
        "propertyNames": { "$ref": "#" },
        "const": true,
        "enum": {
            "type": "array",
            "items": true,
            "minItems": 1,
            "uniqueItems": true
        },
        "type": {
            "anyOf": [
                { "$ref": "#/definitions/simpleTypes" },
                {
                    "type": "array",
                    "items": { "$ref": "#/definitions/simpleTypes" },
                    "minItems": 1,
                    "uniqueItems": true
                }
            ]
        },
        "format": { "type": "string" },
        "contentMediaType": { "type": "string" },
        "contentEncoding": { "type": "string" },
        "if": { "$ref": "#" },
        "then": { "$ref": "#" },
        "else": { "$ref": "#" },
        "allOf": { "$ref": "#/definitions/schemaArray" },
        "anyOf": { "$ref": "#/definitions/schemaArray" },
        "oneOf": { "$ref": "#/definitions/schemaArray" },
        "not": { "$ref": "#" }
    },
    "default": true
}
"##;




fn main () {

    //Minecraft-bedrock-json-schemas\source\compress_specification.json

    let src_dir = Path::new("Minecraft-bedrock-json-schemas\\source\\");


    let comp_spec = fs::read_to_string(src_dir.join("compress_specification.json")).unwrap();

    let compiled_draft_7_schema = JSONSchema::options()
    .with_draft(Draft::Draft7)
    .compile(&serde_json::from_str(DRAFT_7_META_SCHEMA).unwrap()).unwrap();



    let schema: CompressSpec = serde_json::from_str(&comp_spec).unwrap();

    for file in schema.files {

        // read the schema file from src

        // join the src_dir with the file.source file.source is relative to the src_dir

        // print file source


        let json = fs::read_to_string(src_dir.join(&file.source)).unwrap();

        // validate the schema

        let json = serde_json::from_str(&json).unwrap();


        let result = compiled_draft_7_schema.validate(&json);
        if let Err(errors) = result {
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }



        let  schema: Schema = serde_json::from_value(json.clone()).unwrap();

        println!("File: {:?}", file.source);



        process_schema(schema);
    }
}

use syn;
use typify::{TypeSpace,TypeSpaceSettings};

fn process_schema(schema: Schema) {


    let set = TypeSpaceSettings::default();


    let mut ts =  TypeSpace::new(&set);

    ts.add_type(&schema);















}





