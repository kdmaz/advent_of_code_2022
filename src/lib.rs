#![allow(dead_code)]
use std::fs;
mod solutions;

pub fn read_file(folder: &str, day: u8) -> String {
    let filepath = format!("{}/day{:02}.txt", folder, day);
    let error_message = &format!("Could not read from filepath: {}", &filepath);
    fs::read_to_string(&filepath).expect(&error_message)
}