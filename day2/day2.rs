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
    let mut input = io::BufReader::new(file);

    let mut line = String::new();
    let _ = input.read_line(&mut line);
    let ranges = line.split(',').collect::<Vec<&str>>();

    let mut count = 0;
    for range in &ranges {
        let (min, max) = range.trim_end().split_once('-').unwrap();
        let min = min.parse::<u64>().unwrap();
        let max = max.parse::<u64>().unwrap();
        for x in min..=max {
            let str_x = x.to_string();
            let (str_1, str_2) = str_x.split_at(str_x.len()/2);
            if str_1.len() != str_2.len() {
                continue
            }
            if str_1 == str_2 {
                count += x;
            }
        }
    }
    println!("{}", count);

    let mut count = 0;
    let mut invalid = vec!();
    for range in ranges {
        let (min, max) = range.trim_end().split_once('-').unwrap();
        let min = min.parse::<u64>().unwrap();
        let max = max.parse::<u64>().unwrap();
        for x in min..=max {
            if invalid.contains(&x) {
                continue
            }
            let str_x = x.to_string();
            for split_len in 1..str_x.len()/2+1 {
                let mut strs = str_x.chars()
                    .collect::<Vec<char>>()
                    .chunks(split_len)
                    .map(|c| c.iter().collect::<String>())
                    .collect::<Vec<String>>();
                strs.sort();
                let str1 = &strs[0];
                let filtered = strs.clone().into_iter()
                    .filter(|x| x == str1)
                    .collect::<Vec<String>>();
                if strs.len() == filtered.len() {
                    invalid.push(x);
                    println!("match!: {}", x);
                    count += x;
                    break
                }
            }
        }
    }
    println!("{}", count);
    Ok(())
}
