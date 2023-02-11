
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn open(name: String , text: String) {
    let path = Path::new(name.as_str());
    let display = path.display();

    // open file 
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}",display,why),
        Ok(file) => file,
    };

    // write the text to file
    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't wirte ot {}: {}",display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    }

}