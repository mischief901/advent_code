use std::{
    env,
    io::{self, BufRead},
    fs::File,
    error::Error,
    string::String,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    println!("input_file: {}", input_file);

    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);

    let mut lines = vec!();
    for line in input.lines() {
        lines.push(line.unwrap());
    }
    todo!()
    Ok(())
}
