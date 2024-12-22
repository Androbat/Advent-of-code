use std::{fs::File, io::{self, BufRead, BufReader}};

pub struct ProcessFile {
    file: File,
}

impl ProcessFile {
    pub fn new(file_path: &str) -> io::Result<Self> {
        match File::open(file_path) {
            Ok(file) => Ok(Self { file }),
            Err(err) => {
                println!("Unable to open file due to: {}", err);
                std::process::exit(1);
            }
        }
    }

    pub fn parse_sequence(&self, line: &str) -> Vec<i32> {
        line.split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect()
    }

    pub fn is_sequence_safe(&self, sequence: &[i32]) -> bool {
        // sequence.windows(2) -> devides the contents
        // of the data structure, array slice for this casa,
        // into two [1, 2] and get those values.
        for window in sequence.windows(2) {
            let difference = window[1] - window[0];
            if difference.abs() > 3 || difference == 0 {
                return false;
            }
        }
        true
    }

    // Modify this function. 
    // At any change the buffer could not work or read the file. So that must be handled too.
    pub fn process(&self) -> io::Result<i32> {
        let mut counter = 0;
        let reader = BufReader::new(&self.file);
        for (_number, line) in reader.lines().enumerate() {
            match line {
                Ok(content) => {
                    let sequence = self.parse_sequence(&content);
                    match self.is_sequence_safe(&sequence){
                       true => counter += 1,
                       false => ()
                    };
                }
                Err(_) => todo!(),
            };
        }
        Ok(counter)
    }
}