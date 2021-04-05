use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufReader, Write};

use java_properties;

mod string_utils;

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
    print_and_flush("Enter the full path of the properties file you want to convert to yaml: ");

    let file;
    loop {
        let user_input = get_user_input();

        if user_input == "exit" {
            println!("Exiting properties converter...");
            std::process::exit(0)
        }

        println!("Trying to open file: {}", user_input);

        file = match File::open(&user_input) {
            Ok(file) => file,
            Err(e) => {
                println!("File could not be read! Is the path valid? Are the rights of the current user sufficient? \n{}\n", e);
                print_and_flush("Enter the full path of the properties file you want to convert to yaml: ");
                continue
            }
        };
        break;
    }

    let properties_map = match java_properties::read(BufReader::new(file)) {
        Ok(props) => props,
        Err(e) => {
            panic!("Failed to parse properties! Do file contents violate Java properties syntax? \n {}", e);
        }
    };

    println!("Keys read from file: ");
    for key in properties_map.keys() {
        println!("{}", key);
    }

}

fn print_and_flush(message: &str) {
    print!("{}", message);
    io::stdout().flush().unwrap();
}

fn get_user_input() -> String {
    let mut input_file = String::new();
    io::stdin().read_line( &mut input_file).unwrap();
    String::from(input_file.trim())
}
