use std::fs::File;
use std::io::{BufReader, BufRead, Error};
#[derive(Default)]
pub struct Model {
    verts_:Vec<[f32;3]>,
    faces_:Vec<Vec<usize>>,
}

impl Model {
    pub fn from(filename:&str) -> Result<Model, Error> {
        let mut model = Model::default();
        let path = filename;
        let input = File::open(path)?;
        let buffered = BufReader::new(input);
        for line in buffered.lines() {
            // println!("{}", line?);
            let line = line.unwrap();
            let line = line.trim();
            let strs:Vec<&str> = line.split(' ').collect();
            if strs[0].eq("v") {
                model.verts_.push(
                    [
                        strs[1].parse::<f32>().unwrap(), 
                        strs[2].parse::<f32>().unwrap(), 
                        strs[3].parse::<f32>().unwrap(), 
                    ]
                )
            } else if strs[0].eq("f") {
                let mut face:Vec<usize> = vec![];
                for i in 1..=3 {
                    let vunit:Vec<&str> = strs[i].split('/').collect();
                    let v:usize = vunit[0].parse().unwrap();
                    let v:usize = v - 1;
                    let vt:usize = vunit[1].parse().unwrap();
                    let _vt:usize = vt - 1; 
                    let vn:usize = vunit[2].parse().unwrap();
                    let _vn:usize = vn - 1;
                    face.push(v);
                }
                model.faces_.push(face);
            }
        }
        Ok(model)
    }

    pub fn nverts(&self) -> usize {
        self.verts_.len()
    }

    pub fn nfaces(&self) -> usize {
        self.faces_.len()
    }

    pub fn vert(&self, index:usize) -> &[f32;3] {
        self.verts_.get(index).unwrap()
    }

    pub fn face(&self, index:usize) -> &Vec<usize> {
        self.faces_.get(index).unwrap()
    }
}