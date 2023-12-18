use std::io::*;
use std::{fs::File, path::Path};

fn main() {
    let Ok(lines) = read_lines("src/2.txt") else {
        return;
    };
    let mut total = 0;
    for line in lines {
        let Ok(ip) = line else { continue };
        total += power(&ip);
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

fn power(line: &str) -> i32 {
    let mut red_cubes: Option<i32> = None;
    let mut green_cubes: Option<i32> = None;
    let mut blue_cubes: Option<i32> = None;

    let (_, games) = line.split_once(':').unwrap();

    let games = games.split(|s| s == ',' || s == ';');
    let audit = |a:i32, c: &mut Option<i32>| {
        if c.is_none() { *c = Some(a); return }
        if a > c.unwrap() { *c = Some(a); }
    };

    games.for_each(|g| {
        let (amount, color) = g
            .trim()
            .split_once(' ')
            .expect("duh");
        let amount = amount.parse::<i32>().unwrap();

        match color {
            "red" => audit(amount, &mut red_cubes),
            "green" => audit(amount, &mut green_cubes),
            "blue" => audit(amount, &mut blue_cubes),
            _ => panic!()
        }
    });

    red_cubes.unwrap_or_default()
        * green_cubes.unwrap_or_default()
        * blue_cubes.unwrap_or_default()
}
