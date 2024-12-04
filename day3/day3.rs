use std::{
    env,
    io::{self, Read},
    fs::File,
    error::Error,
};



fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    println!("input_file: {}", input_file);

    let mut file = File::open(&input_file)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut total = 0;
    while let Some((mult, rest)) = scan_str(buffer) {
        total += mult;
        buffer = rest;
        let length = buffer.len();
        println!("running total: {total}");
        println!("remaining: {length}");
    }
    println!("final total {total}");
    Ok(())
}

fn scan_str(string: String) -> Option<(u64,String)> {
    match string.split_once("mul(") {
        Some((_, start)) => {
            let mut a = 0;
            let mut b = 0;
            if let Some((pnum, rest)) = start.split_once(",") {
                if pnum.len() <= 3 && pnum.len() > 0 {
                    match pnum.parse::<u64>() {
                        Ok(num) => {
                            a = num;
                            match rest.find(")") {
                                Some(num) if num <= 3 => {
                                    if let Some((pnum, rest)) = rest.split_once(")") {
                                        if pnum.len() <= 3 && pnum.len() > 0 {
                                            match pnum.parse::<u64>() {
                                                Ok(num) => {
                                                    b = num;
                                                    let total = a * b;
                                                    println!("mul({a},{b}) = {total}");
                                                    Some((total, rest.to_string()))
                                                }
                                                Err(_) => {
                                                    println!("no match");
                                                    if let Some((test, _)) = rest.split_at_checked(6) {
                                                        println!("                                                  mul({a},{test}");
                                                    }
                                                    Some((0, rest.to_string()))
                                                }
                                            }
                                        } else {
                                            println!("no match");
                                            if let Some((test, _)) = rest.split_at_checked(6) {
                                                println!("                                                  mul({a},{test}");
                                            }

                                            Some((0, rest.to_string()))
                                        }
                                    } else {
                                        println!("no match");
                                        if let Some((test, _)) = rest.split_at_checked(6) {
                                            println!("                                                  mul({a},{test}");
                                        }
                                        
                                        Some((0, rest.to_string()))
                                    }
                                }
                                else3 => {
                                    println!("                                                  no match: {else3:?}");
                                    if let Some((test, _)) = rest.split_at_checked(6) {
                                        println!("                                                  mul({a},{test}");
                                    }
                                    Some((0, rest.to_string()))
                                }
                            }
                        }
                        Err(_) => {
                            println!("no match");
                            if let Some((test, _)) = rest.split_at_checked(6) {
                                println!("                                                  mul({pnum},{test}");
                            }

                            Some((0, rest.to_string()))
                        }
                    }
                } else {
                    println!("no match");
                    if let Some((test, _)) = rest.split_at_checked(6) {
                        println!("                                                  mul({test}");
                    }

                    Some((0, rest.to_string()))
                }
            } else {
                Some((0, start.to_string()))
            }
        }
        None => {
            None
        }
    }
}                
/*            match start.split_once(")") {
                Some((possible, rest)) => {
                    println!("{:?}", possible);
                    if let Some((a, b)) = possible.split_once(",") {
                        if a.len() <= 3 && b.len() <= 3 {
                            let a = a.parse::<u64>().unwrap();
                            let b = b.parse::<u64>().unwrap();
                            let total = a * b;
                            println!("{a} * {b} = {total}");
                            Some((total, rest.to_string()))
                        } else {
                            Some((0, rest.to_string()))
                        }
                    } else {
                        Some((0, rest.to_string()))
                    }
                }
                None => {
                    None
                }
            }
        }
        None => {
            None
        }
    }
    
}
*/
