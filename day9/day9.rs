use std::{
    env,
    io::{self, BufRead},
    fs::File,
    error::Error,
    collections::VecDeque,
};

/// Area of an axis‑aligned rectangle whose opposite corners are
/// `(x1,y1)` and `(x2,y2)`.  The +1 accounts for the inclusive grid.
fn area((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    let x = x2.abs_diff(x1) + 1;
    let y = y2.abs_diff(y1) + 1;
    x * y
}

/// Debug helper – prints the board (optional, keep commented out for speed).
#[allow(dead_code)]
fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        println!("{:?}", row);
    }
}

/// Draw a vertical segment (including the two end‑points) on `board`.
fn go_vertical(board: &mut Vec<Vec<char>>, x: usize, (y1, y2): (usize, usize)) {
    board[x][y1] = '#';
    board[x][y2] = '#';
    if y1 < y2 {
        for y in y1 + 1..y2 {
            board[x][y] = 'X';
        }
    } else {
        for y in y2 + 1..y1 {
            board[x][y] = 'X';
        }
    }
}

/// Draw a horizontal segment (including the two end‑points) on `board`.
fn go_horizontal(board: &mut Vec<Vec<char>>, y: usize, (x1, x2): (usize, usize)) {
    board[x1][y] = '#';
    board[x2][y] = '#';
    if x1 < x2 {
        for x in x1 + 1..x2 {
            board[x][y] = 'X';
        }
    } else {
        for x in x2 + 1..x1 {
            board[x][y] = 'X';
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // ---------------------------------------------------------------
    // 1️⃣ Load the list of red tiles (the “corners”)
    // ---------------------------------------------------------------
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);

    // Each line is "x,y". The order of the lines is the order of the loop.
    let corners: Vec<(usize, usize)> = input
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut parts = l.split(',').map(|c| c.parse::<usize>().unwrap());
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            (x, y)
        })
        .collect();

    // ---------------------------------------------------------------
    // 2️⃣ Bounding box – needed to allocate the board
    // ---------------------------------------------------------------
    let (min_x, max_x, min_y, max_y) = corners.iter().fold(
        (usize::MAX, 0, usize::MAX, 0),
        |(minx, maxx, miny, maxy), &(x, y)| {
            (
                minx.min(x),
                maxx.max(x),
                miny.min(y),
                maxy.max(y),
            )
        },
    );
    println!("min: {}, {}", min_x, min_y);
    println!("max: {}, {}", max_x, max_y);

    // ---------------------------------------------------------------
    // Helper functions – draw the red/green loop (unchanged logic).
    // ---------------------------------------------------------------
    // (go_vertical and go_horizontal already defined above)

    // ---------------------------------------------------------------
    // 3️⃣ Build the board with a 1‑cell padding on all sides.
    // ---------------------------------------------------------------
    let pad: usize = 1;
    let width = max_x + 2 * pad + 1;   // +1 because coordinates are inclusive
    let height = max_y + 2 * pad + 1;
    let mut board: Vec<Vec<char>> = vec![vec!['.'; height]; width];

    // Helper to shift original coordinates into the padded board.
    let shift = |(x, y): (usize, usize)| (x + pad, y + pad);

    println!("drawing board");
    // Draw loop using shifted coordinates.
    let mut prev = shift(corners[0]);
    for &c in corners.iter().skip(1) {
        let cur = shift(c);
        if prev.0 == cur.0 {
            go_vertical(&mut board, prev.0, (prev.1, cur.1));
        } else {
            go_horizontal(&mut board, prev.1, (prev.0, cur.0));
        }
        prev = cur;
    }
    // close the loop (wrap back to first point)
    let first = shift(corners[0]);
    if prev.0 == first.0 {
        go_vertical(&mut board, prev.0, (prev.1, first.1));
    } else {
        go_horizontal(&mut board, prev.1, (prev.0, first.0));
    }

    // ---------------------------------------------------------------
    // 4️⃣ Flood‑fill from (0,0) – guaranteed outside of padded area.
    // ---------------------------------------------------------------
    println!("painting board");
    let mut visited = vec![vec![false; height]; width];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((0, 0));
    visited[0][0] = true;

    while let Some((x, y)) = queue.pop_front() {
        if board[x][y] == '.' {
            board[x][y] = 'B';
            if x > 0 && !visited[x - 1][y] {
                visited[x - 1][y] = true;
                queue.push_back((x - 1, y));
            }
            if x + 1 < width && !visited[x + 1][y] {
                visited[x + 1][y] = true;
                queue.push_back((x + 1, y));
            }
            if y > 0 && !visited[x][y - 1] {
                visited[x][y - 1] = true;
                queue.push_back((x, y - 1));
            }
            if y + 1 < height && !visited[x][y + 1] {
                visited[x][y + 1] = true;
                queue.push_back((x, y + 1));
            }
        }
    }

    // ---------------------------------------------------------------
    // 5️⃣ Scan every pair of red tiles (the '#') and keep two maxima:
    //    * max_area_part1 – part 1 (only the two corners must be red)
    //    * max_area_part2 – part 2 (the whole rectangle must stay inside)
    // ---------------------------------------------------------------
    let mut max_area_part1 = 0usize; // part 1
    let mut max_area_part2 = 0usize; // part 2

    for (i, &p1) in corners.iter().enumerate() {
        for &p2 in &corners[i + 1..] {
            let a = area(p1, p2);
            // ---------- Part 1 – only corners must be red ----------
            if a > max_area_part1 {
                max_area_part1 = a;
            }

            // ---------- Part 2 – rectangle must be fully inside ----------
            if a <= max_area_part2 {
                continue;
            }

            // Shift points for padded board
            let (x1, y1) = (p1.0 + pad, p1.1 + pad);
            let (x2, y2) = (p2.0 + pad, p2.1 + pad);
            let xr = if x1 < x2 { x1..=x2 } else { x2..=x1 };
            let yr = if y1 < y2 { y1..=y2 } else { y2..=y1 };

            let mut ok = true;
            'scan: for x in xr.clone() {
                for y in yr.clone() {
                    if board[x][y] == 'B' {
                        ok = false;
                        break 'scan;
                    }
                }
            }
            if ok {
                max_area_part2 = a;
            }
        }
    }

    // ---------------------------------------------------------------
    // 6️⃣ Report the answers
    // ---------------------------------------------------------------
    println!("Part 1 – max area (red corners only): {}", max_area_part1);
    println!("Part 2 – max area (red + green interior): {}", max_area_part2);

    Ok(())
}
