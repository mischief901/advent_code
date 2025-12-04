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

    let mut rows = vec!();
    for line in input.lines() {
        let row = line?;
        rows.push(row); 
    }
    let mut board =
        rows.iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut i = 0;
    let height = board.len();
    let width = board[0].len();
    println!("width: {width}");
    println!("height: {height}");
    let mut free = 0;
    while i <= width - 1 {
        let mut j = 0;
        while j <= height - 1 {
            let mut rolls = 0;
            if board[i][j] != '@' {
                j += 1;
                //println!("skip");
                continue
            }
            println!("checking: ({}, {})", i, j);
            println!("spot: {}", board[i][j]);

            if i < width - 1 && board[i+1][j] == '@' {
                rolls += 1;
            }
            if i < width - 1 && j < height - 1 && board[i+1][j+1] == '@' {
                rolls += 1;
            }
            if i < width - 1 && j > 0 && board[i+1][j-1] == '@' {
                rolls += 1;
            }
            if i > 0 && board[i-1][j] == '@' {
                rolls += 1
            }
            if i > 0 && j < height - 1 && board[i-1][j+1] == '@' {
                rolls += 1
            }
            if i > 0 && j > 0 && board[i-1][j-1] == '@' {
                rolls += 1
            }
            if j < height - 1 && board[i][j+1] == '@' {
                rolls += 1
            }
            if j > 0 && board[i][j-1] == '@' {
                rolls += 1
            } 
            if rolls < 4 {
                //println!("free: ({i},{j})");
                free += 1;
            }
            j += 1;
        }
        i += 1;
    }
    println!("free: {}", free);



    
    let mut free = 0;
    loop {
        let mut freed = vec!();
        let mut i = 0;
        while i <= width - 1 {
            let mut j = 0;
            while j <= height - 1 {
                let mut rolls = 0;
                if board[i][j] != '@' {
                    j += 1;
                    //println!("skip");
                    continue
                }
                //println!("checking: ({}, {})", i, j);
                //println!("spot: {}", board[i][j]);

                if i < width - 1 && board[i+1][j] == '@' {
                    rolls += 1;
                }
                if i < width - 1 && j < height - 1 && board[i+1][j+1] == '@' {
                    rolls += 1;
                }
                if i < width - 1 && j > 0 && board[i+1][j-1] == '@' {
                    rolls += 1;
                }
                if i > 0 && board[i-1][j] == '@' {
                    rolls += 1
                }
                if i > 0 && j < height - 1 && board[i-1][j+1] == '@' {
                    rolls += 1
                }
                if i > 0 && j > 0 && board[i-1][j-1] == '@' {
                    rolls += 1
                }
                if j < height - 1 && board[i][j+1] == '@' {
                    rolls += 1
                }
                if j > 0 && board[i][j-1] == '@' {
                    rolls += 1
                } 
                if rolls < 4 {
                    println!("free: ({i},{j})");
                    free += 1;
                    freed.push((i, j));
                    board[i][j] = '.';
                }
                j += 1;
            }
            i += 1;
        }
        if freed.len() == 0 {
            break
        }
    }
    println!("free: {}", free);
    Ok(())
}
