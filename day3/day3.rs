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

    let mut result = 0;
    for bank in lines.clone() {
        if bank.is_empty() {
            continue
        }
        let bats = bank.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let mut i = 0;
        let mut j = 1;
        let mut large = bats[i];
        let mut small = bats[j];
        while i < j && j < bats.len() - 1 {
            if bats[i] < bats[j] {
                i = j;
                j += 1;
                large = bats[i];
                small = bats[j];
            } else if small < bats[j+1] {
                j += 1;
                small = bats[j];
            } else {
                j += 1
            }
        }
        let value = format!("{large}{small}");
        let value = value.parse::<u32>().unwrap();
        result += value;
    }
    println!("{result}");

    
    let mut result = 0;
    for bank in lines {
        if bank.is_empty() {
            continue
        }
        let copy = bank.clone();
        let bats = bank.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();

        let mut k = 12;
        let mut nums = vec!();
        let mut next_start = 0;
        while k > 0 {
            let mut j = next_start;
            let mut largest = bats[next_start];
            let mut largest_index = next_start;
            while j < bats.len() - k {
                if largest < bats[j+1] {
                    j += 1;
                    largest = bats[j];
                    largest_index = j;
                } else {
                    j += 1;
                }
            }
            next_start = largest_index + 1;
            nums.push(largest);
            k -= 1;
        }
        let value = nums.into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join("");
        let value = value.parse::<u64>().unwrap();
        result += value;
    }
    println!("{result}");


    Ok(())
}
