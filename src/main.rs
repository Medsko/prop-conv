mod string_utils;

use std::io;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::ops::Index;
use std::collections::hash_map::RandomState;
use java_properties::read;

struct PropKey {
    name_part: String,
    children: HashMap<String, PropKey>,
    value: Option<String>
}

impl PropKey {

    pub fn new(name_part: &str) -> PropKey {
        let name_part = name_part.to_string();
        let children = HashMap::new();
        PropKey { name_part, children, value: None }
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn name_part(&self) -> &String {
        &self.name_part
    }

    pub fn children(&self) -> &HashMap<String, PropKey, RandomState> {
        &self.children
    }

    pub fn value(&self) -> &Option<String> {
        &self.value
    }
}

fn main() {

    println!("Welcome to properties converter!");
    let mut file;
    loop {
        print!("Enter the full path of the properties file you want to convert to yaml: ");
        let input_file = get_user_input();

        println!("Input: {}", input_file);

        file = match File::open(&input_file.trim()) {
            Ok(file) => file,
            Err(e) => {
                println!("File could not be read! Is the path valid? Are the rights of the current user sufficient? \n{}", e);
                continue
            }
        };
        break;
    }
    let properties_map = match read(BufReader::new(file)) {
        Ok(props) => props,
        Err(e) => {
            println!("Failed to parse properties! Do file contents violate Java properties syntax? \n {}", e);
            panic!()
        }
    };

    println!("Keys read from file: ");
    for key in properties_map.keys() {
        println!("{}", key);
    }

}

fn get_user_input() -> String {
    let mut input_file = String::new();
    io::stdin().read_line( &mut input_file);
    input_file
}
