// In The Name Of Allah

use std;

mod open_file;
fn main() {
    print!("Enter a file name: ");

    let mut name = String::new();
    let mut text = String::new();
    
    std::io::stdin()
        .read_line(&mut name)
        .expect("Error!");

    print!("Enter a text: ");
    std::io::stdin()
        .read_line(&mut text)
        .expect("Error!");
    
    
    
    open_file::open(name, text);

}
