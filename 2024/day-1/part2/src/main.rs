use std::fs::File;
use std::io::{ self, BufRead, BufReader};
use std::collections::HashMap;

pub fn open_file(filename: &str) -> io::Result<BufReader<File>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok((reader))
}

pub fn frequency_per_row() -> io::Result<i32> {
    let file = open_file("sample.test.txt")?;
    let mut freq_counter: HashMap<i32, i32> = HashMap::new();
    let mut right_column: Vec<i32> = Vec::new();
    let mut left_column: Vec<i32> = Vec::new();
    let mut score = 0;
    

    for line in file.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        

        if let (Some(str_left), Some(str_right)) = (parts.get(0), parts.get(1)) {
            if let (Ok(left), Ok(right)) = (str_left.parse::<i32>(), str_right.parse::<i32>()) {
                right_column.push(right);
                left_column.push(left);
                freq_counter.entry(left).or_insert(0);
            }

        }
    }


    for left in &left_column {
        let count = right_column.iter().filter(|&right| right == left).count();
        score += left * count as i32;
    }
    
    
    Ok(score)
}



fn main() {
    println!("{:?}", frequency_per_row());
}
