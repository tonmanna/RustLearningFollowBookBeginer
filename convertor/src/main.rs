use std::env;
use utility;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    utility::read_key()
}
