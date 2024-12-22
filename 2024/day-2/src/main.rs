mod process_file;
use process_file::ProcessFile;


fn main() -> std::io::Result<()> {
    let process = ProcessFile::new("./base-test.txt").expect("Failed to create ProcessFile");
    let safe_count = process.process()?;
    println!("Number of safe sequences: {}", safe_count);
    Ok(())
}
