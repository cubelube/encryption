mod encryption;

use encryption::encrypt;
use std::io::stdin;

fn main() {
    let mut unconverted = String::new();

    println!("Enter the text to be converted:");
    
    stdin()
        .read_line(&mut unconverted)
        .expect("Failed");

    let converted = encrypt(&unconverted);

    println!("{}", converted);
}