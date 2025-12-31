use std::fs::read_to_string;
use std::path::PathBuf;

pub fn read_input(filename: &str) -> String {
    match read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join(filename),
    ) {
        Ok(content) => content,
        Err(_) => panic!("Input does not exist!"),
    }
}
