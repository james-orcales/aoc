use std::io::*;
use std::{fs::File, path::Path};

const RED_CUBES: i32 = 12;
const GREEN_CUBES: i32 = 13;
const BLUE_CUBES: i32 = 14;

fn main() {
    let Ok(lines) = read_lines("src/2.txt") else {
        return;
    };
    let mut total = 0;
    for line in lines {
        let Ok(ip) = line else { continue };
        let Some(id) = possible_game_id(&ip) else {
            continue;
        };
        total += id;
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

fn possible_game_id(line: &str) -> Option<i32> {
    let (id, games) = line.split_once(':').unwrap();

    let impossible = games.split(|s| s == ',' || s == ';').any(|c| -> bool {
        let (num, color) = c
            .trim()
            .split_once(' ')
            .expect("duh");

        let num = num.parse::<i32>().unwrap();

        if num < 12 {
            false
        } else {
            match color {
                "red" => if num > RED_CUBES { true } else { false }
                "green" => if num > GREEN_CUBES { true } else { false }
                "blue" => if num > BLUE_CUBES { true } else { false }
                _ => { println!("tite {}", color); false } 
            }
        }
    });

    // ( "Game", "{id}" )
    let id = id.split_once(' ').unwrap().1.parse::<i32>().unwrap();

    if impossible { None } else { Some(id) } 
}

