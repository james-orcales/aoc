use std::io::*;
use std::{fs::File, path::Path};

const WORD_NUMBER: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const WORD_NUMBER_REV: [&str; 10] = [
    "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

fn main() {
    let Ok(lines) = read_lines("src/1.txt") else { return };
    let mut total = 0;
    for line in lines {
        if let Ok(ip) = line {
            total += calibration_value(&ip);
        }
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

fn calibration_value(line: &str) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut second_digit: Option<u32> = None;
    let mut chs = line.chars();
    let mut word_buf = String::new();

    while let Some(ch) = chs.next() {
        if ch.is_ascii_digit() {
            first_digit = Some(ch.to_digit(10).expect("yup"));
            break
        }

        word_buf.push(ch);
        if word_buf.len() < 3 { continue }
        for (i, wn) in WORD_NUMBER.iter().enumerate() {
            if word_buf.contains(wn) { 
                first_digit = Some(i as u32); 
                word_buf.clear()
            }
        } 
        if first_digit.is_some() { break }
    }

    while let Some(ch) = chs.next_back() {
        if ch.is_ascii_digit() {
            second_digit = Some(ch.to_digit(10).expect("yup"));
            break
        }

        word_buf.push(ch);
        if word_buf.len() < 3 { continue }
        for (i, wn) in WORD_NUMBER_REV.iter().enumerate() {
            if word_buf.contains(wn) { 
                second_digit = Some(i as u32); 
            }
        } 
        if second_digit.is_some() { break }
    }

    if second_digit.is_none() {second_digit = first_digit}
    first_digit.unwrap_or_default() * 10 + second_digit.unwrap_or_default()
}
