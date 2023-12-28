use std::io::*;
use std::{fs::File, path::Path};


fn main() { 
    let Ok(lines) = read_lines("src/3.txt") else {
        return;
    };
    let mut total: u32 = 0;
    let mut prev_line = None;

    for line in lines {
        let Ok(line) = line else { return };
        let mut pepe = parse_line(&line);
        if prev_line.is_none() {
            prev_line = Some(pepe);
            continue
        }
        let tite = parse_total(&mut pepe, prev_line.unwrap());
        total += tite;
        println!("{}: {}", line, tite);

        prev_line = Some(pepe);
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

#[derive(Default)]
struct Line {
    part_numbers: Vec<PartNumber>,
    sym_indices: Vec<SymIndex>,
}

impl Line {
    fn new() -> Self {
        Default::default()
    }
}

type SymIndex = usize;

struct PartNumber {
    number: u32,
    zone: (usize, usize),
}

#[derive(Default)]
struct PartNumberBuilder {
    number_string: String,
    left_boundary: usize,
}

impl PartNumberBuilder {
    fn new(left_boundary: usize) ->  Self {
        Self {
            number_string: Default::default(),
            left_boundary,
        }
    }

    fn build(self, right_boundary: usize) -> PartNumber {
        let number = self.number_string.parse::<u32>().expect("Not u32 string");
        PartNumber {
            number,
            zone: (self.left_boundary, right_boundary)
        }
    }
}


fn parse_line(line_str: &str) -> Line {
    let mut ci = line_str.char_indices().peekable();
    let mut line = Line::new(); 
    let mut pnb: Option<PartNumberBuilder> = None;
    
    while let Some((i, ch)) = ci.next() {
        let inside_number = pnb.is_some();
        if ch.is_ascii_punctuation() {
            if inside_number {
                line.part_numbers.push(pnb.unwrap().build(i));
                pnb = None;
            }

            if ch != '.' { line.sym_indices.push(i); }

            let Some(&(_, next_ch)) = ci.peek() else { continue };
            if next_ch.is_ascii_digit() { 
                pnb = Some(PartNumberBuilder::new(i)); 
            }
        } else if ch.is_ascii_digit() {
            if !inside_number { pnb = Some(PartNumberBuilder::new(i)) }
            pnb.as_mut().unwrap().number_string.push(ch);
        }
    }

    if pnb.is_some() {
        line.part_numbers.push(pnb.unwrap().build(line_str.len()-1));
    }

    line
}

fn parse_total(cur_line: &mut Line, mut prev_line: Line) -> u32 {
    let mut total = 0;

    prev_line.sym_indices.extend(cur_line.sym_indices.iter());
    let combined_si = prev_line.sym_indices;

    'outer: for p in prev_line.part_numbers {
        for i in &combined_si {
            if i >= &p.zone.0 && i <= &p.zone.1 {
                total += p.number;
                continue 'outer
            }
        }
    }

    let mut cur_retain = vec![];
    'outer: while let Some(p) = cur_line.part_numbers.pop() {
        for i in &combined_si {
            if i >= &p.zone.0 && i <= &p.zone.1 {
                total += p.number;
                continue 'outer
            }
        }
        cur_retain.push(p);
    }

    cur_line.part_numbers = cur_retain;

    total
}
