use std::{
    env,
    error::Error,
};
fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_unstable_by_key(|&(s, _)| s);

    let mut merged = Vec::new();
    let mut cur = ranges[0];
    for &(s, e) in &ranges[1..] {
        if cur.1 + 1 >= s {
            if e > cur.1 {
                cur.1 = e;
            }
        } else {
            merged.push(cur);
            cur = (s, e);
        }
    }
    merged.push(cur);
    merged
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    println!("input_file: {}", input_file);

    let contents = std::fs::read_to_string(input_file)?;
    let mut parts = contents.splitn(2, "\n\n");
    let ranges_part: Vec<&str> = parts
        .next()
        .unwrap_or("")
        .lines()
        .collect();
    let items_part: Vec<&str> = parts
        .next()
        .unwrap_or("")
        .lines()
        .collect();
    let ranges: Vec<(u64, u64)> = ranges_part
        .iter()
        .filter_map(|line| {
            let mut split = line.split('-');
            let start = split.next()?.parse::<u64>().ok()?;
            let end = split.next()?.parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect();
    // Parse items into u64 values
    let items: Vec<u64> = items_part
        .iter()
        .filter_map(|line| line.parse::<u64>().ok())
        .collect();
    println!("Ranges: {:?}", ranges);
    println!("Items: {:?}", items);

    let mut fresh = 0;
    for item in items {
        for (x,y) in ranges.clone().into_iter() {
            if (x..=y).contains(&item) {
                fresh += 1;
                break
            }
        }
    }
    println!("{}", fresh);

    let items = merge_ranges(ranges);
    println!("{:?}", items);
    let mut number = 0;
    for (start, end) in items {
        number += end - start + 1;
    }
    println!("{}", number);
    Ok(())
}
