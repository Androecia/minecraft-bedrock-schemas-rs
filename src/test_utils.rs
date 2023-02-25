use walkdir::WalkDir;

//TODO: make something for tests to verify it wont serialize incorrect files


pub fn test_serde_json_from_files_in_path<T>(paths: Vec<String>)
where
    T: serde::de::DeserializeOwned+ ?Sized + serde::Serialize,
{
    let mut file_paths = Vec::new();
    for path in paths {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                file_paths.push(entry.path().to_str().unwrap().to_string());
            }
        }
    }


    let mut files:Vec<T> = Vec::new();
    // parse the json files
    for file_name in file_paths {
        let file = std::fs::File::open(file_name.clone()).unwrap();




        let  reader = std::io::BufReader::new(file);

//        println!("Parsing file: {}", file_name);

        let deser_file: T = match serde_json::from_reader(reader) {
            Ok(file) => file,
            Err(e) => panic!("error parsing json: {}, \n File: {}", e, file_name),
        };

        let file = std::fs::File::open(file_name.clone()).unwrap();
        let  reader = std::io::BufReader::new(file);

        let original_file: serde_json::Value = match serde_json::from_reader(reader) {
            Ok(file) => file,
            Err(e) => panic!("error parsing json: {}, \n File: {}", e, file_name),
        };




        // deser_file as serde_json::Value

        let ser_file = serde_json::to_string_pretty(&deser_file).unwrap();

       // println!("Serializing file: {}", ser_file);



        let deser_file: serde_json::Value = match serde_json::from_str(&ser_file) {
            Ok(file) => file,
            Err(e) => panic!("error parsing json: {}, \n File: {}", e, file_name),
        };






        /// pretty print the json eq

        //assert_eq!(original_file, deser_file, "error serializing json: \n File: {}", file_name);

        if original_file != deser_file {
            // show original file value and deser file value in the error with the file name and an error message

            panic!("\n------------------\nThe serialized value != the original value \n------------------\nFile: {}\n------------------\nOriginal File: \n{:#?}\n------------------\n Serialized File:\n {:#?}\n------------------\n", file_name, original_file, deser_file);
        }



       // files.push(file);
    }


}
