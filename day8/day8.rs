use std::{
    env,
    io::{self, BufRead},
    fs::File,
    error::Error,
    collections::HashMap,
    fmt::{self, Display},
    process,
};

#[derive(Clone, Debug, )]
struct Board {
    board: HashMap<String, Vec<(usize,usize)>>,
    size_x: usize,
    size_y: usize,
}

impl Board {
    fn new(vec_board: Vec<Vec<String>>) -> Self {
        let size_x = vec_board.len();
        let size_y = vec_board[0].len();
        let mut board: HashMap<String, Vec<(usize,usize)>> = HashMap::new();
        let mut row = 0;
        while row < vec_board.len() {
            let mut col = 0;
            while col < vec_board.len() {
                let spot = &vec_board[row][col];
                if spot != "." {
                    if board.contains_key(spot) {
                        match board.get_mut(spot) {
                            Some(pos) => {
                                pos.push((row, col));
                            }
                            None => {
                                eprintln!("Error inserting board.");
                                process::exit(1);
                            }
                        }
                    } else {
                        board.insert(spot.to_string(), vec!((row, col)));
                    }
                }
                col += 1;
            }
            row += 1;
        }
        Self {
            board,
            size_x,
            size_y,
        }
    }
    fn find_antinodes(&mut self) -> Vec<(usize,usize)> {
        let mut antinodes: Vec<(usize,usize)> = vec!();
        for (_ch, spots) in self.board.iter() {
            // look at each character on the board individually.
            let mut board = spots.clone();
            let mut index_base = 0;
            while index_base < board.len() - 1 {
                let mut index_test = index_base + 1;
                while index_test < board.len() {
                    if !antinodes.contains(&board[index_base]) {
                        antinodes.push(board[index_base].clone());
                    }
                    if !antinodes.contains(&board[index_test]) {
                        antinodes.push(board[index_test].clone());
                    }
                    let (first_x, first_y) = board[index_base];
                    let (second_x, second_y) = board[index_test];
                    println!("({first_x}, {first_y})");
                    println!("({second_x}, {second_y})");
                    let (diff_x, diff_y): (i64, i64) = ((first_x as i64 - second_x as i64), (first_y as i64 - second_y as i64));
                    println!("diff: ({diff_x:?}, {diff_y:?})");
                    let mut pos1: (i64, i64) = (first_x as i64 + diff_x, first_y as i64 + diff_y);
                    println!("pos1: {pos1:?}");
                    let mut pos2: (i64, i64) = (second_x as i64 - diff_x, second_y as i64 - diff_y);
                    println!("pos2: {pos2:?}");
                    while let Some(pos) = self.on_board(pos1) {
                        if !antinodes.contains(&pos) {
                            antinodes.push(pos);
                        }
                        pos1 = (pos1.0 as i64 + diff_x, pos1.1 as i64 + diff_y);
                    }
                    while let Some(pos) = self.on_board(pos2) {
                        if !antinodes.contains(&pos) {
                            antinodes.push(pos);
                        }
                        pos2 = (pos2.0 as i64 - diff_x, pos2.1 as i64 - diff_y);
                    }
                    index_test += 1;
                }
                index_base += 1;
            }
        }
        self.board.insert("#".to_string(), antinodes.clone());
        antinodes
    }

    // Check if position is valid and converts it to usize (could be negative up to this point).
    fn on_board(&self, pos: (i64,i64)) -> Option<(usize, usize)> {
        let (x, y) = pos;
        if x < 0 || x >= self.size_x as i64 {
            return None
        }
        if y < 0 || y >= self.size_y as i64 {
            return None
        }
        Some((x as usize, y as usize))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut board = vec![vec![".".to_string(); self.size_y]; self.size_x];
        for (key, val) in self.board.iter() {
            for (x, y) in val {
                let key = key.clone();
                board[*x][*y] = key.to_string();
            }
        }
        for line in board.iter() {
            let line = line.join("");
            write!(f, "{}\n", line)?;
        }
        write!(f, "")
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    println!("input_file: {}", input_file);

    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);
    let mut raw_board: Vec<Vec<String>> = vec!();
    for line in input.lines() {
        let line = line?;
        let line = line.trim()
            .split("")
            .filter(|c| *c != "")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        raw_board.push(line);
    }
    let mut board = Board::new(raw_board);
    println!("{board}");

    let antinodes = board.find_antinodes();
    println!("{antinodes:?}");
    println!("Antinodes: {}", antinodes.len());
    println!("{board}");
    Ok(())
}
