pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;

use std::fs::read_to_string;
use std::path::PathBuf;

pub fn solve<T, F>(day: u8, problem: u8, f: F) -> T
where
    F: FnOnce(&str) -> T,
{
    let input_path = PathBuf::from(format!(
        "{}/inputs/{:02}-{:02}.txt",
        env!("CARGO_MANIFEST_DIR"),
        day,
        problem
    ));
    let content = read_to_string(&input_path).expect("File with input data should exist");
    f(&content)
}
