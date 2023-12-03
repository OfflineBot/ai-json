use std::{fs::File, io::{Read, Write}};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct SingleVec<T> {
    pub data: T
}

impl<T> SingleVec<T> 
where
    T: for<'a> Deserialize<'a> + Serialize
{
    pub fn read_vec1(path: &str) -> Vec<T> {
        let mut file = match File::open(path) {
            Ok(data) => data,
            Err(_) => panic!("Could not open File!")
        };
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        serde_json::from_str(&buffer).unwrap()
    }

    pub fn read_vec2(path: &str) -> Vec<Vec<T>> {
        let mut file = match File::open(path) {
            Ok(data) => data,
            Err(_) => panic!("Could not open File!")
        };
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        serde_json::from_str(&buffer).unwrap()
    }

    pub fn write_vec1(path: &str, data: Vec<T>) {
        let mut file = match File::create(path) {
            Ok(data) => data,
            Err(_) => panic!("Could not open File!")
        };
        let buffer = serde_json::to_string_pretty(&data).unwrap();
        file.write_all(buffer.as_bytes()).unwrap();
    }
    
    pub fn write_vec2(path: &str, data: Vec<Vec<T>>) {
        let mut file = match File::create(path) {
            Ok(data) => data,
            Err(_) => panic!("Could not open File!")
        };
        let buffer = serde_json::to_string_pretty(&data).unwrap();
        file.write_all(buffer.as_bytes()).unwrap();
    }
}