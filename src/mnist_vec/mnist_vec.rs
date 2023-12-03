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
    T: for<'a> Deserialize<'a> + Clone
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

    pub fn min_len(&self) -> usize {
        let output = self.output.clone();
        let mut min_val = usize::MAX;

        if output.pic_0.len() < min_val {
            min_val = output.pic_0.len();
        }

        if output.pic_1.len() < min_val {
            min_val = output.pic_1.len();
        }
        if output.pic_2.len() < min_val {
            min_val = output.pic_2.len();
        }
        if output.pic_3.len() < min_val {
            min_val = output.pic_3.len();
        }
        if output.pic_4.len() < min_val {
            min_val = output.pic_4.len();
        }
        if output.pic_5.len() < min_val {
            min_val = output.pic_5.len();
        }
        if output.pic_6.len() < min_val {
            min_val = output.pic_6.len();
        }
        if output.pic_7.len() < min_val {
            min_val = output.pic_7.len();
        }
        if output.pic_8.len() < min_val {
            min_val = output.pic_8.len();
        }
        if output.pic_9.len() < min_val {
            min_val = output.pic_9.len();
        }
       
        min_val
    }

    pub fn combine(&self, count: usize) -> (Vec<Vec<T>>, Vec<Vec<T>>) {
        let mut min_len = self.min_len();
        if count > min_len {
            min_len = count;
            println!("There are not enough pictures for given count! continueing with: {}", min_len);
        }

        let mut input_vec: Vec<Vec<T>> = vec![];
        let mut output_vec: Vec<Vec<T>> = vec![];
        
        for i in 0..min_len {
            input_vec.push(self.input.pic_0[i].clone());
            output_vec.push(self.output.pic_0[i].clone());
        }
        for i in 0..min_len {
            input_vec.push(self.input.pic_1[i].clone());
            output_vec.push(self.output.pic_1[i].clone());
        }
        for i in 0..min_len {
            input_vec.push(self.input.pic_2[i].clone());
            output_vec.push(self.output.pic_2[i].clone());
        }
        for i in 0..min_len {
            input_vec.push(self.input.pic_3[i].clone());
            output_vec.push(self.output.pic_3[i].clone());
        }
        for i in 0..min_len {
            input_vec.push(self.input.pic_4[i].clone());
            output_vec.push(self.output.pic_4[i].clone());
        }
        for i in 0..min_len {
            input_vec.push(self.input.pic_5[i].clone());
            output_vec.push(self.output.pic_5[i].clone());
        }
        for i in 0..min_len {
            input_vec.push(self.input.pic_6[i].clone());
            output_vec.push(self.output.pic_6[i].clone());
        }
        for i in 0..min_len {
            input_vec.push(self.input.pic_7[i].clone());
            output_vec.push(self.output.pic_7[i].clone());
        }
        for i in 0..min_len {
            input_vec.push(self.input.pic_8[i].clone());
            output_vec.push(self.output.pic_8[i].clone());
        }
        for i in 0..min_len {
            input_vec.push(self.input.pic_9[i].clone());
            output_vec.push(self.output.pic_9[i].clone());
        }

        (input_vec, output_vec)
    }
}