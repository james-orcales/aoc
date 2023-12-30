mod matrix {
    #[derive(Default)]
    pub struct Matrix {
        pub rows: Vec<Vec<char>>
    }

    impl Matrix {
        pub fn new() -> Self {
            Matrix::default()
        }

        pub fn cells(&self) -> impl Iterator<Item = ((usize, usize), &char)> {
            self.rows
                .iter()
                .enumerate()
                .flat_map(|(y, v)| v.iter().enumerate().map(move |(x, v)| ((x, y), v)))
        }

        pub fn add_row(&mut self, r: Vec<char>) {
            self.rows.push(r)
        }

        pub fn get_cell(&self, x: usize, y: usize) -> Option<&char> {
            self.rows.get(y)?.get(x)
        }

        pub fn parse_part_number(&self, x: usize, y: usize) -> Option<(u32, Vec<(usize, usize)>)> {
            let mut num = String::new();
            let mut checked_indices =  vec![];

            let ch = self.get_cell(x, y)?;
            if !ch.is_ascii_digit() { return None };
            num.push(*ch);

            let mut px = x;
            while let Some(pxs) = px.checked_sub(1) {
                let Some(ch) = self.get_cell(pxs, y) else { break };
                
                if !ch.is_ascii_digit() { break };

                num.insert(0, *ch);
                checked_indices.push((pxs, y));
                px = pxs;
            };

            let mut nx = x;
            while let Some(nxa) = nx.checked_add(1) {
                let Some(ch) = self.get_cell(nxa, y) else { break };

                if !ch.is_ascii_digit() { break };

                num.push(*ch);
                checked_indices.push((nxa, y));
                nx = nxa;
            };

            Some((num.parse::<u32>().unwrap(), checked_indices))
        }
    }
}

use std::{fs::File, path::Path, io::*};
use matrix::Matrix;

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

// DUPLICATION IS ALLOWED
fn main() {
    let Ok(lines) = read_lines("src/3.txt") else {
        return;
    };
    let mut matrix: Matrix = Matrix::new();
    let mut total: u32 = 0;
    for line in lines {
        let Ok(line) = line else { return };
        matrix.add_row(line.chars().collect());
    }

    let cells = matrix.cells();
    for ((x, y), ch) in cells {
        if *ch != '*' { continue }
        let mut neighbors = vec![];
        let mut checked_indices = vec![];
        for (dx, dy) in DIRS {
            let Some(x) = x.checked_add_signed(dx) else { continue };
            let Some(y) = y.checked_add_signed(dy) else { continue };
            if checked_indices.contains(&(x, y)) { continue };
            let Some(neighbor) = matrix.get_cell(x, y) else { continue };
            if *neighbor == '.' { continue }

            let (n, ci) = matrix.parse_part_number(x, y).unwrap();
            neighbors.push(n);
            checked_indices.extend(ci);

        }
        if neighbors.len() != 2 { continue }
        total += neighbors[0] * neighbors[1];
    }
    println!("{}", total);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

