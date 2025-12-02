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
    
    let mut dial: i16 = 50;
    let mut zeros = 0;
    for line in lines.clone() {
        let (dir, dist) = line.split_at(1);
        let count = dist.parse::<i16>().unwrap();

        if dir == "L" {
            dial -= count;
            dial += 100;
            dial %= 100;
        } else {
            dial += count;
            dial %= 100;
        }
        if dial == 0 {
            zeros += 1;
        }
    }
    println!("count: {}", zeros);

    let mut dial2: i16 = 50;
    let mut zeros2 = 0;
    for line in lines {
        let (dir, dist) = line.split_at(1);
        let count = dist.parse::<i16>().unwrap();
        if dir == "L" {
            let mut next = dial2 - count;
            if next <= 0 {
                while next <= 0 {
                    zeros2 += 1;
                    next += 100;
                }
            }
            if dial2 == 0 {
                zeros2 -= 1;
            }
            dial2 = next % 100;
        } else {
            let mut next = dial2 + count;
            if next >= 100 {
                while next >= 100 {
                    zeros2 += 1;
                    next -= 100;
                }
            }
            dial2 = next;
        }
    }    
    println!("count2: {}", zeros2);
    
    Ok(())
}
