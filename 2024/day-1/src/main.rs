use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Read};
use std::path::Path;

mod utils;


fn main() {
    let diff = utils::calculate_total_diff("input.txt");
    match diff {
        Ok(result) => {
            println!("The total difference is: {:?}", result);
        }
        Err(err) => {
            eprintln!("Error occurred: {:?}", err);
        }
    }
}