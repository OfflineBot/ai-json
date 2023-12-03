use std::{fs::File, io::{Write, Read}};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct InputData<T> {
    pub input: Vec<Vec<T>>,
    pub output: Vec<Vec<T>>,
}

impl<T> InputData<T> 
where
    T: for<'a> Deserialize<'a>
{

    pub fn empty(path: &str) {
        let mut file = match File::create(path) {
            Ok(data) => data,
            Err(_) => panic!("Could not create File!"),
        };
        let json_data: InputData<u8> = InputData {
            input: vec![],
            output: vec![],
        };
        let json_str = serde_json::to_string_pretty(&json_data).unwrap();
        file.write_all(json_str.as_bytes()).unwrap();
    }

    pub fn read(path: &str) -> InputData<T> {
        let mut file = match File::open(path) {
            Ok(data) => data,
            Err(_) => panic!("Could not open File!")
        };
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let out: InputData<T> = serde_json::from_str(&buffer).unwrap();
        out
    }

}