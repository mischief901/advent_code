use std::{
    env,
    fs::File,
    io::{self, BufRead},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);
    let lines: Vec<Vec<String>> = input
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    let mut total_part1: u64 = 0;
    let mut col = 0;
    while col < lines[0].len() {
        let op = lines[lines.len() - 1][col].as_str();
        let mut values: Vec<u64> = Vec::new();
        for row in 0..lines.len() - 1 {
            values.push(lines[row][col].parse::<u64>()?);
        }
        let result: u64 = if op == "+" { values.iter().copied().sum() } else { values.iter().copied().product() };
        total_part1 += result;
        col += 1;
    }
    println!("Part 1 total: {}", total_part1);


    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);
    let mut lines: Vec<String> = input.lines()
        .map(|x| x.unwrap())
        .collect();
    let lines: Vec<Vec<char>> = lines.into_iter()
        .map(|x| {
            x.chars()
                .collect()
        })
        .collect();
    println!("lines: {:?}", lines);

    let mut transformed: Vec<Vec<char>> = vec!{vec!{}; lines[0].len()};
    println!("transformed blank: {:?}", transformed);
    
    for col in 0..lines[0].len() {
        for row in 0..lines.len() {
            transformed[col].push(lines[row][col]);
        }
    }
    println!("transformed filled: {:?}", transformed);

    let mut total = 0;
    
    let mut i = 0;
    let mut nums = vec!();
    let mut op: Option<char> = None;
    for item in &transformed {
        println!("op: {:?}", op);
        if op.is_none() {
            op = Some(item[item.len()-1]);
            println!("op: {:?}", op);
        }
        let num = item[0..item.len()-1].into_iter().collect::<String>();
        let num = num.trim();
        println!("num: {}", num);
        if !num.is_empty() {
            nums.push(num.parse::<u64>().unwrap());
        } else {
            let result: u64 = if op == Some('+') { nums.iter().copied().sum() } else { nums.iter().copied().product() };
            println!("results: {}", result);
            total += result;
            op = None;
            nums = vec!();
        }
    }
    let result: u64 = if op == Some('+') { nums.iter().copied().sum() } else { nums.iter().copied().product() };
    println!("results: {}", result);
    total += result;

    println!("total: {}", total);
    Ok(())
}
