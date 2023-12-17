use std::io::*;
use std::{fs::File, path::Path};

fn total_cv() {
    if let Ok(lines) = read_lines("src/1.txt") {
        let mut total = 0;
        for line in lines {
            if let Ok(ip) = line {
                total += calibration_value(&ip);
            }
        }
        println!("{}", total);
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn calibration_value(line: &str) -> u32 {
    let mut cv = 0;
    let mut chs = line.chars();

    if let Some(d) = chs.find(|ch| ch.is_numeric()) {
        cv += d.to_digit(10).unwrap() * 10;
    };

    cv += if let Some(d) = chs.rfind(|ch| ch.is_numeric()) {
        d.to_digit(10).unwrap()
    } else {
        cv / 10
    };

    cv
}

