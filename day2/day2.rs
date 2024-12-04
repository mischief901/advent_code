use std::{
    env,
    io::{self, BufRead},
    fs::File,
    error::Error,
};



fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    println!("input_file: {}", input_file);

    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);

    let mut safe_total = 0;

    for line in input.lines() {
        let line = line?.to_owned();
        println!("{line}");
        let num_iter: Vec<u64> = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        let mut i = 0;
        if safe(num_iter.clone()) {
            safe_total += 1;
        } else {
            while i < num_iter.len() {
                let mut new_nums = num_iter.clone();
                let _ = new_nums.remove(i);
                println!("{new_nums:?}");
                if safe(new_nums) {
                    safe_total += 1;
                    break
                }
                i += 1;
            }
        }
    }
    println!("{safe_total}");
    Ok(())
}

fn safe(num_iter: Vec<u64>) -> bool {
    let mut increasing = None;
    let mut previous = None;
    let mut safe = true;
    for num in num_iter {
        match previous {
            None => {
                previous = Some(num);
                continue;
            }
            Some(prev) => {
                match increasing {
                    // The first 2 nums when we don't know if it's increasing or decreasing.
                    None => {
                        let diff = num.abs_diff(prev);
                        println!("diff: {diff}");
                        // Check that the diff is 0 or greater than 3. If it is exit the line.
                        if diff > 3 || diff == 0 {
                            safe = false;
                            println!("too big");
                            break
                        }
                        if num < prev {
                            increasing = Some(false);
                        } else {
                            increasing = Some(true);
                        }
                        previous = Some(num);
                        println!("{previous:?}");
                        continue
                    }
                    Some(increase) => {
                        let diff = num.abs_diff(prev);
                        println!("diff: {diff}");
                        // Check that the diff is 0 or greater than 3. If it is exit the line.
                        if diff > 3 || diff == 0 {
                            println!("too big");
                            safe = false;
                            break
                        }
                        if increase && num < prev {
                            println!("switched directions");
                            safe = false;
                            break
                        }
                        if increase && num > prev {
                            previous = Some(num);
                        } else if num < prev && ! increase {
                            previous = Some(num);
                        } else {
                            println!("switched directions");
                            safe = false;
                            break
                        }
                        println!("{previous:?}");
                    }
                }
            }
        }
    }
    safe
}
