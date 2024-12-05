use std::{
    collections::HashMap,
    env,
    io::{self, BufRead},
    fs::File,
    error::Error,
    process,
    cmp::Ordering,
};



fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    println!("input_file: {}", input_file);

    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);

    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut correct = 0;
    let mut corrected = 0;

    for line in input.lines() {
        let line = line?;
        if let Some((a, b)) = line.split_once("|") {
            let a = a.parse::<u64>().unwrap();
            let b = b.parse::<u64>().unwrap();
            if let Some(orders) = rules.get_mut(&a) {
                orders.push(b);
            } else {
                rules.insert(a.to_owned(), vec![b]);
            }
        } else if line.len() > 0 {
            let mut line_split: Vec<u64> = line.split(",").map(|l| l.parse::<u64>().unwrap()).collect();
            println!("{line_split:?}");
            if check(&line_split, &rules) {
                correct += line_split[line_split.len()/2];
            } else {
                println!("{line_split:?}");
                line_split.sort_by(|a, b| {
                    let rules = rules.clone();
                    if let Some(rest) = rules.get(a) {
                        if rest.contains(b) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    } else {
                        Ordering::Equal
                    }
                });
                if check(&line_split, &rules) {
                    println!("Corrected! {line_split:?}");
                    corrected += line_split[line_split.len()/2];
                } else {
                    println!("still not correct: {line_split:?}");
                    process::exit(1)
                }
            }
        } else {
            continue
        }
    }
    println!("{correct}");
    println!("{corrected}");
    Ok(())
}

fn check(line_split: &[u64], rules: &HashMap<u64, Vec<u64>>) -> bool {
    let mut checked = vec!();
    let mut valid = true;
    for c in line_split {
        if !valid {
            break
        }
        match rules.get(&c) {
            None => {
                checked.push(c.clone());
            }
            Some(rest) => {
                for seen in &checked {
                    if rest.contains(&seen) {
                        valid = false;
                        println!("found invalid line");
                        println!("already seen {seen} for {c}");
                        break
                    }
                }
                checked.push(c.clone());
            }
        }
    }
    valid
}
