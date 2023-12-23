use std::io::BufRead;
use std::{fs::File, path::Path};
use std::{io::*, vec};

#[derive(Default, Clone)]
struct Part {
    number: u32,
    left_boundary: usize,
    right_boundary: usize,
}

#[derive(Default)]
struct PartBuilder {
    number_string: String,
    left_boundary: Option<usize>,
    right_boundary: Option<usize>,
    tagged: bool,
}

impl PartBuilder {
    fn new() -> Self {
        PartBuilder::default()
    }

    fn num_string(&mut self) -> &mut String {
        &mut self.number_string
    }

    fn number_string_as_u32(&self) -> u32 {
        self.number_string.parse::<u32>().expect("Expected u32 string")
    }

    fn set_left_boundary(&mut self, left_boundary: usize) {
        self.left_boundary = Some(left_boundary);
    }

    fn set_right_boundary(&mut self, right_boundary: usize) {
        self.right_boundary = Some(right_boundary);
    }

    fn tag(&mut self) {
        self.tagged = true;
    }

    fn tagged(&self) -> bool {
        self.tagged
    }

    fn is_in_num(&self) -> bool {
        self.left_boundary.is_some() && self.right_boundary.is_none()
    }

    fn is_outside_num(&self) -> bool {
        !self.is_in_num()
    }

    fn clear(&mut self) {
        self.number_string.clear();
        self.left_boundary = None;
        self.right_boundary = None;
        self.tagged = false;
    }

    fn build(&self) -> Part {
        Part {
            number: PartBuilder::number_string_as_u32(&self),
            left_boundary: self.left_boundary.expect("Set left boundary"),
            right_boundary: self.right_boundary.expect("Set right boundary"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct SymbolIndex(usize);

#[derive(Default, Clone)]
struct Line {
    parts: Vec<Part>,
    symbol_indices: Vec<SymbolIndex>,
    total: u32,
}

fn main() {
    let Ok(lines) = read_lines("src/3.txt") else { return };

    let mut total = 0;
    let mut prev_line: Option<Line> = None;

    for line in lines {
        let Ok(ip) = line else { continue };

        let l = parse_line(&ip, prev_line.clone());
        total += l.total;

        prev_line = Some(l);
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

fn parse_line(line_str: &str, prev_line: Option<Line>) -> Line {
    let mut line = Line::default();
    let mut pb = PartBuilder::new();
    let pl = prev_line.unwrap_or_default();

    let mut chs = line_str.chars().enumerate().peekable();
    while let Some((i, ch)) = chs.next() {
        let Some(&(next_i, next_ch)) = chs.peek() else {
            if pb.is_outside_num() && ch.is_ascii_punctuation() && ch != '.' {
                line.symbol_indices.push(SymbolIndex(i));
                pb.tag()
            } else if ch.is_ascii_digit() {
                pb.num_string().push(ch);
                pb.set_right_boundary(i);
                line.parts.push(pb.build());
                pb.clear()
            }
            break
        };

        // guarantees that left_boundary is Some when ch is ascii digit
        if ch.is_ascii_digit() {
            pb.num_string().push(ch);

            if pb.is_outside_num() { pb.set_left_boundary(i); }
            if pl.symbol_indices.contains(&SymbolIndex(i)) { pb.tag() }
            if !next_ch.is_ascii_punctuation() { continue }
            if next_ch != '.' || pl.symbol_indices.contains(&SymbolIndex(next_i)){ 
                pb.tag() 
            }

            if pb.tagged() { 
                line.total += pb.number_string_as_u32();
                pb.clear();
                continue
            }

            pb.set_right_boundary(i+1);
            line.parts.push(pb.build());
            pb.clear()
                // safe to assume we are not inside a number
        } else if ch.is_ascii_punctuation() {
            if ch != '.' { line.symbol_indices.push(SymbolIndex(i)); }

            if !next_ch.is_ascii_digit() {
                continue
            }

            pb.set_left_boundary(i);
            if ch == '.' && !pl.symbol_indices.contains(&SymbolIndex(i)) {
                continue
            };
            pb.tag()
        } 
    }
    
    for p in pl.parts {
        for i in &line.symbol_indices {
            if i >= &SymbolIndex(p.left_boundary) && i <= &SymbolIndex(p.right_boundary){
                line.total += p.number;
                break;
            }
        }
    }

    println!("{}: {}", line_str, line.total);
    line
}
