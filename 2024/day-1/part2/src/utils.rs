use std::fs::File;
use std::io::{ self, BufRead, BufReader};


struct ReadFile {
    path: &str
}

impl ReadFile {
    fn new(self, file: ReadFile) -> Self {
        Self { file }
    }

    fn read_file(file: ReadFile) -> io::Result<()>{
        let file = File::open(&self.file)?;
        Ok(())
    }
}