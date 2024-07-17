use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub mod types;

pub fn load_file(path: &PathBuf) -> types::json::JsonFormat {
    let mut file = File::open(path).unwrap();
    let mut file_str = String::new();
    file.read_to_string(&mut file_str)
        .expect("unable to read file to string");
    let deserialized: types::json::JsonFormat = serde_json::from_str(&file_str).unwrap();
    deserialized
}

pub fn load_bytes(bytes: Vec<u8>) -> types::json::JsonFormat {
    let deserialized: types::json::JsonFormat = serde_json::from_slice(bytes.as_slice()).unwrap();
    deserialized
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
