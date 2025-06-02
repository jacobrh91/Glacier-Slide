use serde;

use std::fs;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

use super::point::Point;

#[derive(Serialize, Deserialize)]
pub struct Level {
    pub cols: usize,
    pub rows: usize,
    pub start: Point,
    pub end: Point,
    pub rocks: Vec<Point>,
    solutions: Option<Vec<String>>,
}

fn get_data_directory() -> String {
    let pathbuf = fs::canonicalize("data/").expect("Could not find the data directory.");
    pathbuf.into_os_string().into_string().unwrap()
}

pub fn read_level_data(name: &str) -> Result<Level, Box<dyn Error>> {
    let path = format!("{}/levels/{}.json", get_data_directory(), name);
    println!("{}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u: Level = serde_json::from_reader(reader)?;
    Ok(u)
}
