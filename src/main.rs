use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Ranges {
    prefixes: Vec<Prefix>
}

#[derive(Serialize, Deserialize, Debug)]
struct Prefix {
    ip_prefix: String,
    region: String,
    service: String
}

fn main() {
    println!("Starting..");
    
    let mut file = File::open("C:\\Temp\\ip-ranges.json").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let deserialized: Ranges = serde_json::from_str(&contents).unwrap();

    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    for entry in deserialized.prefixes {        
        if !result.contains_key(&entry.region) {
            result.insert(entry.region.to_string(), Vec::new());
        }
        result.get_mut(&entry.region).unwrap().push(entry.ip_prefix);
    }

    let serialized = serde_json::to_string_pretty(&result).unwrap();
    fs::write("C:\\Temp\\ip_ranges_by_region.json", serialized).expect("Unable to write file");
    println!("Completed");
}