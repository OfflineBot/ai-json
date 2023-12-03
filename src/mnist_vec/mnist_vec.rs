use std::{fs::File, io::{Read, Write}};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Pictures<T> {
    pub pic_0: Vec<Vec<T>>,
    pub pic_1: Vec<Vec<T>>,
    pub pic_2: Vec<Vec<T>>,
    pub pic_3: Vec<Vec<T>>,
    pub pic_4: Vec<Vec<T>>,
    pub pic_5: Vec<Vec<T>>,
    pub pic_6: Vec<Vec<T>>,
    pub pic_7: Vec<Vec<T>>,
    pub pic_8: Vec<Vec<T>>,
    pub pic_9: Vec<Vec<T>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MnistData<T> {
    pub input: Pictures<T>,
    pub output: Pictures<T>
}

impl<T> MnistData<T> 
where
    T: for<'a> Deserialize<'a>
{
    pub fn empty(path: &str) {
        let mut file = match File::create(path) {
            Ok(data) => data,
            Err(_) => panic!("Could not create File!")
        };
        let out_pic: Pictures<u8> = Pictures {
            pic_0: vec![],
            pic_1: vec![],
            pic_2: vec![],
            pic_3: vec![],
            pic_4: vec![],
            pic_5: vec![],
            pic_6: vec![],
            pic_7: vec![],
            pic_8: vec![],
            pic_9: vec![],
        };

        let out_struct: MnistData<u8> = MnistData { input: out_pic.clone(), output: out_pic };

        let out_str = serde_json::to_string_pretty(&out_struct).unwrap();
        file.write_all(out_str.as_bytes()).unwrap();
    }

    pub fn read(path: &str) -> MnistData<T> {
        let mut file = match File::open(path) {
            Ok(data) => data,
            Err(_) => panic!("Could not open File!")
        };
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let out: MnistData<T> = serde_json::from_str(&buffer).unwrap();
        out
    }
}