use std::io;
use std::io::Read;

pub fn read_string() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    return guess;
}

pub fn read_key() {
    let mut guess = [0; 10];
    io::stdin().read(&mut guess).unwrap();
}

pub fn print_seperator(){
    println!("------------------------------------")
}