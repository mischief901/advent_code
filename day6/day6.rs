use std::{
    env,
    io::{self, BufRead},
    fs::File,
    error::Error,
};

#[derive(Clone, Debug)]
pub enum Direction {
    None,
    Up,
    Left,
    Right,
    Down,
}
impl Direction {
    fn new() -> Self {
        Direction::None
    }
    fn turn(&mut self) -> () {
        match self {
            Direction::None => *self = Direction::None,
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }
    fn step(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let (mut row, mut col) = pos;
        let mut row = row as i64;
        let mut col = col as i64;
        match self {
            Direction::None => Some(pos),
            Direction::Up => {
                row -= 1;
                if row < 0 {
                    None
                } else {
                    Some((row as usize, col as usize))
                }
            }
            Direction::Down => {
                Some(((row + 1) as usize, col as usize))
            }
            Direction::Left => {
                col -= 1;
                if col < 0 {
                    None
                } else {
                    Some((row as usize, col as usize))
                }
            }
            Direction::Right => {
                Some((row as usize, (col + 1) as usize))
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    println!("input_file: {}", input_file);

    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);

    let mut board: Vec<Vec<String>> = vec!();
    for line in input.lines() {
        let line = line?;
        let row: Vec<String> = line.split("")
            .skip_while(|c| *c == "")
            .map(|s| s.to_string())
            .collect();
        board.push(row.to_owned());
    }

    let mut pos = (0,0);
    let mut direction = Direction::new();
    let mut row = 0;
    let row_len = board.len();
    let col_len = board[0].len();
    while row < row_len {
        let mut col = 0;
        let line = &board[row];
        while col < col_len {
            let c = &line[col];
            if c == "<" {
                direction = Direction::Left;
                pos = (row, col);
                break
            } else if c == ">" {
                direction = Direction::Right;
                pos = (row, col);
                break
            } else if c == "^" {
                direction = Direction::Up;
                pos = (row, col);
                break
            } else if c == "v" {
                direction = Direction::Down;
                pos = (row, col);
                break
            } else {
                col += 1;
            }
        }
        row += 1;
    }
    //println!("Board Size: {row_len}, {col_len}");
    //println!("starting pos: {pos:?}");
    //println!("starting dir: {direction:?}");

    // For part2.
    let board2 = board.clone();
    let mut pos_step_2 = pos.clone();
    let mut direction_step_2 = direction.clone();
    
    let mut steps = 1;
    let (row, col) = pos;
    board[row][col] = "X".to_string();
    loop {
        if let Some(next_pos) = direction.step(pos) {
            let (next_row, next_col) = next_pos;
            if next_row < 0 || next_row >= row_len {
                break
            }
            if next_col < 0 || next_col >= col_len {
                break
            }
            let space = &mut board[next_row][next_col];
            if space == "#" {
                //println!("turning");
                direction.turn();
            } else {
                pos = next_pos;
            }
            if space == "." {
                board[next_row][next_col] = "X".to_string();
                steps += 1;
            } else {
                //println!("skipping count: {space:?}");
            }
            //println!("step taken: {pos:?}");
            //println!("total steps: {steps}");
        } else {
            break
        }
    }
    println!("total steps: {steps}");

    let max_loops = steps.clone();

    let mut loop_count = 0;
    let mut row_block = 0;
    let (row, col) = pos_step_2;
    //println!("pos: {pos:?}");
    while row_block < row_len {
        let mut col_block = 0;
        while col_block < col_len {
            let mut temp_board2 = board2.clone();
            let mut direction2 = direction_step_2.clone();
            let mut steps = 1;
            temp_board2[row][col] = "X".to_string();
            let mut loop_steps = 0;
            temp_board2[row_block][col_block] = "#".to_string();
            //println!("{temp_board2:?}");
            let mut pos2 = pos_step_2.clone();
            loop {
                if let Some(next_pos) = direction2.step(pos2) {
                    
                    let (next_row, next_col) = next_pos;
                    if next_row < 0 || next_row >= row_len {
                        break
                    }
                    if next_col < 0 || next_col >= col_len {
                        break
                    }
                    let space = &mut temp_board2[next_row][next_col];
                    if space == "#" {
                        //println!("turning");
                        direction2.turn();
                    } else {
                        pos2 = next_pos;
                    }
                    if space == "." {
                        temp_board2[next_row][next_col] = "X".to_string();
                        steps += 1;
                    } else {
                        //println!("potential loop: {loop_steps}");
                        loop_steps += 1;
                        
                        if loop_steps > max_loops {
                            //println!("Loop found: {steps}");
                            loop_count += 1;
                            //println!("loop count: {loop_count}");
                            break
                        }
                    }
                } else {
                    break
                }
                //println!("step taken: {pos:?}");
                //println!("total steps: {steps}");
            }
            col_block += 1;
        }
        row_block += 1;
    }
    println!("loops found: {loop_count}");
    
    Ok(())
}
