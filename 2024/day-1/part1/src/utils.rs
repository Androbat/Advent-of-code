use std::fs::File;
use std::io::{ self, BufRead, BufReader};

// THINK OF A REFECTOR
pub fn calculate_total_diff(path: &str) -> io::Result<i32> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    let mut total = 0;


    for line in reader.lines(){
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if let (Some(x_str), Some(y_str)) = (parts.get(0), parts.get(1)) {
            if let (Ok(x), Ok(y)) = (x_str.parse::<i32>(), y_str.parse::<i32>()) {
                col1.push(x);
                col2.push(y);
            }
        }
    }
   

    col1.sort();
    col2.sort();
    
    for (left, right) in col1.iter().zip(col2.iter()){
        let sustract = (right - left).abs();
        total += sustract;
    }



   Ok(total)
}

/*
     ALGORITHM
      - Sort the columns in the file
      - Set a counter to store the sum of every value      - Iterate over every row: 4 5
      - Convert the both rows values in every iteration
      - Sustract both values and sum it in the same iteration.
      - Return the total sum of every value.
    
*/
