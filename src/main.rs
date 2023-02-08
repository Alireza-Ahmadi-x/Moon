// In The Name Of Allah

#[allow(unused_imports)]
use colored::Colorize;

use std::io;

fn main() {
    println!("Enter a text");

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Erro!");
    
    color(text);

}


fn color(x:String) -> (){
    println!("{}",x.red());
}