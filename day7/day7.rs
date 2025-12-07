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

    let board: Vec<Vec<char>> = input.lines()
        .map(|i| i.unwrap())
        .map(|i| i.chars().collect())
        .collect();

    let (start, _) = board[0].iter().enumerate().find(|(_, s)| **s == 'S').unwrap();


    let mut visit_counts: Vec<Vec<usize>> = board.iter().map(|row| vec![0; row.len()]).collect();

    let mut positions = vec![start];
    let mut height = 0;
    let mut splits = 0;
    while height < board.len() - 1 {
        let mut next_positions = Vec::new();
        let mut seen_in_row = std::collections::HashSet::new();
        while let Some(beam) = positions.pop() {
            // Increment visit count for current position at this height
            if let Some(row_counts) = visit_counts.get_mut(height) {
                if let Some(cnt) = row_counts.get_mut(beam) {
                    *cnt += 1;
                }
            }
            if board[height + 1][beam] == '^' {
                // split beam
                if beam > 0 {
                    let left = beam - 1;
                    if seen_in_row.insert(left) { next_positions.push(left); }
                }
                let right = beam + 1;
                if right < board[height + 1].len() {
                    if seen_in_row.insert(right) { next_positions.push(right); }
                }
                splits += 1;
            } else {
                if seen_in_row.insert(beam) { next_positions.push(beam); }
            }
        }
        positions = next_positions;
        height += 1;
    }
    println!("splits: {}", splits);

    let mut ways: Vec<Vec<u128>> = board.iter().map(|row| vec![0u128; row.len()]).collect();
    ways[0][start] = 1;
    for r in 0..board.len()-1 {
        for c in 0..board[r].len() {
            let count = ways[r][c];
            if count == 0 { continue; }
            if board[r+1][c] == '^' {
                // split to left and right in next row
                if c > 0 {
                    ways[r+1][c-1] += count;
                }
                if c+1 < board[r+1].len() {
                    ways[r+1][c+1] += count;
                }
            } else {
                // continue straight down
                ways[r+1][c] += count;
            }
        }
    }
    // Sum ways on the bottom row to get total timelines
    let total_timelines: u128 = ways.last().unwrap().iter().sum();

    println!("timelines: {}", total_timelines);
    Ok(())
}
