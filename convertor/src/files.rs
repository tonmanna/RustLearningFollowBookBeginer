use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn readfile_sync() -> io::Result<()> {
    let file = File::open("foo.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
