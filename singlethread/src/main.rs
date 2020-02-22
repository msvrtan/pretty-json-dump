extern crate serde;
extern crate serde_json;

use serde::Serialize;
use std::fs;
use std::fs::DirEntry;

fn get_files_in_folder(path: String) -> Vec<DirEntry> {
    let paths = fs::read_dir(path).unwrap();
    let mut files = Vec::new();

    for path_result in paths {
        let result = path_result.unwrap();
        let file_type = &result.file_type().unwrap();
        let is_dir = file_type.is_dir();

        if is_dir == true {
            let x = get_files_in_folder(result.path().to_str().unwrap().to_string());
            for val in x {
                files.push(val);
            }
        } else {
            files.push(result);
        }
    }

    files
}

fn reformat(result: DirEntry) {
    println!("Filename: {:?}", &result.file_name().into_string().unwrap());
    let content = content(&result);
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    json.serialize(&mut ser).unwrap();
    let mut pretty = String::from_utf8(ser.into_inner()).unwrap();
    // Add new line to end of file
    pretty.push('\n');

    write(&result, &pretty);
}

pub fn content(file: &DirEntry) -> String {
    fs::read_to_string(file.path()).expect("Something went wrong reading the file")
}

pub fn write(file: &DirEntry, content: &String) {
    fs::write(file.path(), content).expect("Something went wrong writing the file")
}

fn main() {
    let files = get_files_in_folder(String::from("/path/data"));

    for file in files {
        reformat(file);
    }

    println!("Done");
}
