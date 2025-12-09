use std::{
    env,
    io::{self, BufRead},
    fs::File,
    error::Error,
    string::String,
    collections::HashMap,
};

fn get_distance((x1, y1, z1): (i64, i64, i64), (x2, y2, z2): (i64, i64, i64)) -> f64 {
    let x = (x1 - x2).pow(2) as f64;
    let y = (y1 - y2).pow(2) as f64;
    let z = (z1 - z2).pow(2) as f64;
    (x + y + z).sqrt()
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size:   vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        // Path‑compression (iterative version)
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        // compress the path
        while self.parent[x] != x {
            let nxt = self.parent[x];
            self.parent[x] = root;
            x = nxt;
        }
        root
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false; // already together
        }

        // union‑by‑size – keep the larger tree as the new root
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }

    fn component_size(&mut self, x: usize) -> usize {
        let r = self.find(x);
        self.size[r]
    }

    fn root(&mut self, x: usize) -> usize {
        self.find(x)
    }

    pub fn all_component_sizes(&self) -> Vec<usize> {
        self.size
            .iter()
            .enumerate()
            .filter_map(|(i, &sz)| if self.parent[i] == i { Some(sz) } else { None })
            .collect()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let collect = args[2].parse::<usize>().unwrap();

    println!("input_file: {}", input_file);

    let file = File::open(&input_file)?;
    let input = io::BufReader::new(file);

    let points = input.lines()
        .map(|i| i.unwrap())
        .map(|i| {
            let mut dims: Vec<i64> = i.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            let z = dims.pop().unwrap();
            let y = dims.pop().unwrap();
            let x = dims.pop().unwrap();
            (x, y, z)
        })
        .collect::<Vec<(i64, i64, i64)>>();
    //println!("points: {:?}", points);

    let mut circuits = UnionFind::new(points.len());    

    let mut values = vec!();
    
    for (i, point_1) in points.iter().enumerate() {
        for point_2 in &points[i+1..] {
            let distance = get_distance(*point_1, *point_2);
            let distance_2 = get_distance(*point_2, *point_1);
            if distance != 0.0 {
                if distance < distance_2 {
                    values.push((distance, point_1, point_2));
                } else {
                    values.push((distance_2, point_2, point_1));
                }
            }
        }
    }
    
    values.sort_by(|(x, _, _), (y, _, _)| f64::total_cmp(x, y));
    //println!("values:{:?}", values);

    for (_, p1, p2) in values.iter().take(collect) {
        let i1 = points.iter().position(|p| p == *p1).unwrap();
        let i2 = points.iter().position(|p| p == *p2).unwrap();
        circuits.union(i1, i2);
    }

    let mut size = circuits.all_component_sizes();
    size.sort_unstable_by(|a, b| b.cmp(a));
    let answer: u128 = size.iter()
        .take(3)
        .map(|x| *x as u128)
        .product();
    println!("answer: {}", answer);


    let mut circuits2 = UnionFind::new(points.len());
    let mut remaining = points.len();
    for (_, p1, p2) in values {
        let i1 = points.iter().position(|p| p == p1).unwrap();
        let i2 = points.iter().position(|p| p == p2).unwrap();
        if circuits2.union(i1, i2) {
            remaining -= 1;
        }
        if remaining == 1 {
            let (x1, _, _) = p1;
            let (x2, _, _) = p2;
            println!("last: {}", x1 * x2);
            break
        }
    }

    
    
    Ok(())
}
