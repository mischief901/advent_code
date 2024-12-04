use std::{
    collections::HashMap,
    env,
    io::{self, BufRead},
    iter,
    fs::File,
    error::Error,
    process,
};



fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    println!("input_file: {}", input_file);

    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);

    let mut a = vec!();
    let mut b = vec!();

    let mut b_hash = HashMap::new();
    //let mut count = 0;

    for line in input.lines() {
        let line = line?.to_owned();
        let mut items = line.split_whitespace();
        match (items.next(), items.next()) {
            (Some(a_int), Some(b_int)) => {
                a.push(a_int.parse::<u64>()?);
                let b_temp = b_int.parse::<u64>()?;
                if b_hash.contains_key(&b_temp) {
                    let Some(b_count) = b_hash.get_mut(&b_temp) else {panic!()};
                    *b_count += 1;
                } else {
                    b_hash.insert(b_temp, 1);
                }
                b.push(b_temp);
                //count += 1;
            }
            (_, _) => {
                eprintln!("Invalid line");
                process::exit(1);
            }
        }
    }
    //println!("{count:?}");
    a.as_mut_slice().sort();
    b.as_mut_slice().sort();
    let mut total = 0;
    
    for (a, b) in iter::zip(a.clone(), b) {
        total += a.abs_diff(b);
    }
    println!("part 1: {total}");
    total = 0;
    for a_value in a.into_iter() {
        match b_hash.get(&a_value) {
            Some(b_count) => {
                total += a_value * b_count;
            }
            None => {
                ()
            }
        }
    }

    println!("part 2: {total}");
    Ok(())
}
