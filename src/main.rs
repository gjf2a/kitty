use std::env::args;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::io;

fn main() -> io::Result<()> {
    for arg in args().skip(1) {
        let input_file = File::open(arg)?;
        let reader = BufReader::new(input_file);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }
    Ok(())
}
