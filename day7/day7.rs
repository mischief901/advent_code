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

    let mut total = 0;
    fn fork(total: i64, values: &[u64]) -> bool {
//        println!("target: {total}");
//        println!("values: {values:?}");
        if values.len() == 0 {
            total == 0
        } else if values.len() == 1 {
            total == values[0] as i64
        } else {
            let a = values[0];
            let b = values[1];
            let sum = (a + b) as i64;
//            println!("trying sum");
            let mut rem_values = vec!(sum as u64);
            rem_values.extend_from_slice(&values[2..]);
            if fork(total, &rem_values) {
                true
            } else {
//                println!("trying mult");
                let mult = (a * b) as i64;
                let mut rem_values = vec!(mult as u64);
                rem_values.extend_from_slice(&values[2..]);
                if fork(total, &rem_values) {
                    true
                } else {
//                    println!("trying concat");
                    let concat = format!("{a}{b}").parse::<u64>().unwrap();
                    let mut rem_values = vec!(concat);
                    rem_values.extend_from_slice(&values[2..]);
                    fork(total, &rem_values)
                }
            }
        }
    }

    for line in input.lines() {
        let line = line?;
        let mut target: Option<u64> = None;
        let mut values: Vec<u64> = vec!();
        for value in line.split_whitespace() {
            if target.is_none() {
                let sum = value.strip_suffix(":").unwrap()
                    .parse::<u64>()?;
                target = Some(sum);
            } else {
                values.push(value.parse::<u64>()?);
            }
        }
        //println!("target: {target:?}");
        //println!("values: {values:?}");
        let target = target.unwrap();
        if fork(target as i64, &values) {
//            println!("combo successful");
            //for value in values {
            //    total += value;
            //}
            total += target;
//            println!("total: {total}");
        } else {
//            println!("combo not found");
        }
    }
    println!("total: {total}");
    Ok(())
}
