use std::env::args;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    for arg in args().skip(1) {
        let input_file = File::open(arg).unwrap();
        let reader = BufReader::new(input_file);
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }
}
