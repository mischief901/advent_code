use std::{
    env,
    io::{self, BufRead},
    fs::File,
    error::Error,
    collections::VecDeque,
};

fn area((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    let x = x2.abs_diff(x1) + 1;
    let y = y2.abs_diff(y1) + 1;
    x * y
}

fn print_board(board: Vec<Vec<char>>) {
    for row in board {
        println!("{:?}", row);
    }      
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    println!("input_file: {}", input_file);

    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);

    let corners = input.lines()
        .map(|i| i.unwrap())
        .map(|i| {
            let mut corners: Vec<usize> = i.split(',')
                .map(|c| c.parse::<usize>()
                     .unwrap())
                .collect();
            let x = corners.pop().unwrap();
            let y = corners.pop().unwrap();
            (x, y)
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect::<Vec<(usize, usize)>>();
    //println!("{:?}", corners);

    let mut max_area = 0;
    for (i, pos_1) in corners.iter().enumerate() {
        for pos_2 in &corners[i+1..] {
            let a = area(*pos_1, *pos_2);
            if a > max_area {
                //println!("area: {}", a);
                //println!("{:?}, {:?}", pos_1, pos_2);
                max_area = a;
            }
        }
    }

    println!("area: {}", max_area);
    
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;
    for (x, y) in corners.clone().into_iter() {
        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }
    }

    println!("min: {}, {}", min_x, min_y);
    println!("max: {}, {}", max_x, max_y);

    fn go_vertical(board: &mut Vec<Vec<char>>, x: usize, (y1, y2): (usize, usize)) {
        board[x][y1] = '#';
        board[x][y2] = '#';
        if y1 < y2 {
            for y in y1+1..y2 {
                board[x][y] = 'X';
            }
        } else {
            for y in y2+1..y1 {
                board[x][y] = 'X';
            }
        }
    }

    fn go_horizontal(board: &mut Vec<Vec<char>>, y: usize, (x1, x2): (usize, usize)) {
        board[x1][y] = '#';
        board[x2][y] = '#';
        if x1 < x2 {
            for x in x1+1..x2 {
                board[x][y] = 'X';
            }
        } else {
            for x in x2+1..x1 {
                board[x][y] = 'X';
            }
        }
    }

    println!("drawing board");
    let mut board: Vec<Vec<char>> = vec!(vec!('.'; max_y + 1); max_x + 1);
    let mut i = 1;
    let first = corners[0];
    let mut previous = corners[0];
    while i < corners.len() {
        let (x1, y1) = previous;
        let (x2, y2) = corners[i];
        //println!("previous: {:?}", previous);
        //println!("checking: {:?}", corners[i]);
        if x1 == x2 {
            // move vertical
        go_vertical(&mut board, x1, (y1, y2));
        } else if y1 == y2 {
            // move horizontal
        go_horizontal(&mut board, y1, (x1, x2));
        }
        previous = corners[i];
        i += 1;
        if i == corners.len() {
            let (x2, y2) = first;
            let (x1, y1) = previous;
            //println!("previous: {:?}", previous);
            //println!("checking: {:?}", first);
            if x1 == x2 {
                // move vertical
                go_vertical(&mut board, x1, (y1, y2));
            } else if y1 == y2 {
                // move horizontal
                go_horizontal(&mut board, y1, (x1, x2));
            }            
        }
    }

    println!("painting board");
    // Flood‑fill from (0,0) to mark exterior cells.
    // Use a visited bool matrix (O(1) checks) and a VecDeque for BFS – cache‑friendly and allocation‑light.
    let mut visited = vec![vec![false; board[0].len()]; board.len()];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((0, 0));
    visited[0][0] = true;

    while let Some((x, y)) = queue.pop_front() {
        if board[x][y] == '.' {
            // Mark as outside
            board[x][y] = 'B';
            // Explore neighbours if inside bounds and not yet visited
            if x > 0 && !visited[x - 1][y] {
                visited[x - 1][y] = true;
                queue.push_back((x - 1, y));
            }
            if x + 1 < board.len() && !visited[x + 1][y] {
                visited[x + 1][y] = true;
                queue.push_back((x + 1, y));
            }
            if y > 0 && !visited[x][y - 1] {
                visited[x][y - 1] = true;
                queue.push_back((x, y - 1));
            }
            if y + 1 < board[0].len() && !visited[x][y + 1] {
                visited[x][y + 1] = true;
                queue.push_back((x, y + 1));
            }
        }
    }

    //println!("\n\n");
    //print_board(board.clone());
    //println!("\n\n");

    println!("finding max");
    let mut max_area2 = 0;
    for (i, pos_1) in corners.iter().enumerate() {
        'outer: for pos_2 in &corners[i+1..] {
            let a = area(*pos_1, *pos_2);
            //println!("area: {}", a);
            if a > max_area2 {
                // check that the exterior is not any of the points in the square.
                let (x1, y1) = *pos_1;
                let (x2, y2) = *pos_2;
                let range_x = if x1 < x2 { x1..=x2 } else {x2..=x1};
                let range_y = if y1 < y2 { y1..=y2 } else {y2..=y1};
                let mut valid = true;
                //println!("{:?}", &board[range_x.clone()]);
                //println!("{:?}", range_x);
                //println!("{:?}", range_y);
                for x in range_x {
                    for y in range_y.clone() {
                        //println!("checking: {:?}", (x,y));
                        //println!("{}", board[x][y]);
                        if board[x][y] == 'B' {
                            //println!("hit exterior: {:?}", (x,y));
                            valid = false;
                            break 'outer;
                        }
                    }
                }
                if valid {
                    max_area2 = a;
                }
            }
        }
    }

    println!("max 2: {}", max_area2);
    
    Ok(())
}
