use std::io;
use crate::utils::file_operations;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct SBox {
    value: [[u8; 16]; 4],
}

impl SBox {
    pub fn new() -> Self {
        let mut value: [[u8; 16]; 4] = [[0; 16]; 4]; 
        let mut rng = thread_rng();
        
        for row in value.iter_mut() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = j as u8;
            }
            row.shuffle(&mut rng);
        }
        
        SBox {
            value
        }
    }

    pub fn from(values: [[u8; 16]; 4]) -> Self {
        SBox {
            value: values,
        }
    }

    pub fn get_value(&self, row: u8, col: u8) -> u8 {
        self.value[row as usize][col as usize]
    }
    
    pub fn value(&self) -> [[u8; 16]; 4] {
        self.value
    }
}

pub struct SBoxes {
    pub s_boxes: Vec<SBox>,
}

impl SBoxes {
    pub fn new(num_boxes: u8) -> Self {
        let mut s_boxes = Vec::new();
        for _ in 0..num_boxes {
            s_boxes.push(SBox::new());
        }

        SBoxes {
            s_boxes,
        }
    }

    pub fn from(s_boxes: Vec<SBox>) -> Self {
        SBoxes {
            s_boxes,
        }
    }

    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        file_operations::read_substitution_boxes(filename)
    }

    pub fn clone(&self) -> Self {
        let mut s_boxes = Vec::new();
        for s_box in self.s_boxes.iter() {
            s_boxes.push(SBox::from(s_box.value()));
        }

        SBoxes {
            s_boxes,
        }
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        file_operations::save_substitution_boxes_to_file(self.clone(), filename)
    }

    pub fn get_value(&self, s_box: u8, row: u8, col: u8) -> u8 {
        self.s_boxes[s_box as usize].get_value(row, col)
    }
}