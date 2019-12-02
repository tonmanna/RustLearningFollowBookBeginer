mod files;
use std::env;
use std::io::ErrorKind;
use utility;
fn main() {
    let args: Vec<String> = env::args().collect();
    match files::readfile_sync() {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("File not found"),
            ErrorKind::PermissionDenied => println!("Permission not allow"),
            _ => println!("Other error: {:?}", e),
        },
        Ok(v) => println!("Working {:?}", v),
    }
    println!("{:?}", args);
    utility::read_key()
}
