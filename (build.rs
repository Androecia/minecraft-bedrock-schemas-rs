use std::{
    env, fs,
    path::{Path, PathBuf},
};

use schemars::schema::Schema;
use serde_json::Value;
use typify::{TypeSpace, TypeSpaceSettings};

// Recursively walk a directory and return a list of all files in it recursively
fn walk_dir_recursive(dir: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                paths.extend(walk_dir_recursive(&path));
            } else {
                paths.push(path);
            }
        }
    }
    paths
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");



    let schemas :Vec<PathBuf>= walk_dir_recursive(Path::new(
        "C:\\Users\\miner\\OneDrive\\Documents\\GitHub\\Androecia\\FriendConnect-rs\\minecraft-bedrock-schemas-rs\\Minecraft-bedrock-json-schemas",
    ));
    let ignored_root_directories = vec!["source", "test", "tools", ".github", ".vscode", "src"];

    // TODO: Make this an ENV variable
    let n_path_segments_to_remove: u8 = 10;

    //let  paths: Vec<String> = Vec::new();

    for path in schemas {
        if !path.clone().display().to_string().contains(".json") {
            continue;
        }

        //Check if the file is a json file

        let  separator = if cfg!(windows) { "\\" } else { "/" };
        let content = std::fs::read_to_string(path.clone()).unwrap();

        let mut path = path
            .to_str()
            .unwrap()
            .split(separator.clone())
            .skip(n_path_segments_to_remove as usize)
            .collect::<Vec<&str>>();

        if path.len() < 2
            || !path[path.len() - 1].ends_with(".json")
            || ignored_root_directories.contains(&path[0])
        {
            continue;
        }

        let schema: Value =match  serde_json::from_str(&content) {
            Ok(s) => s,
            Err(e) => {
                println!("Error parsing json: {}", e);
                continue;
        }};

        // check if the json is a schema
        if schema.get("$schema").is_none() {
            continue;
        }

        let file_name = path.pop().unwrap().to_string();

        let mut path_new =
            Path::new(&format!(".{}src{}", separator.clone(), separator.clone())).to_path_buf();

        for p in path.iter() {
            path_new.push(p);
        }

        match fs::create_dir_all(&path_new) {
            Ok(_) => {}
            Err(e) => {
                println!("Error creating directory: {}", e);
            }
        }

        path_new.push(file_name.replace(".json", ".rs"));
        println!("Writing to: {}", path_new.display());
        let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

        let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));

        match type_space.add_ref_types(schema.definitions) {
            Ok(_) => {}
            Err(e) => {
                println!("Error adding ref types: {}", e);
            }
        }



        let base_type = &schema.schema;

        // Only convert the top-level type if it has a name
        if (|| base_type.metadata.as_ref()?.title.as_ref())().is_some() {
            let _ = match type_space.add_type(&Schema::Object(schema.schema)) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error adding type: {}", e);
                }
            };
        }
        let contents = format!(
            "{}\n{}",
            "use serde::{Deserialize, Serialize};",
            type_space.to_string()
        );
        fs::write(path_new, contents).unwrap();
    }
}
