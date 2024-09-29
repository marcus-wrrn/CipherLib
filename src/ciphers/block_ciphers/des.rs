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

pub struct ExpansionTable {
    table: [u8; 48],
}

impl ExpansionTable {
    pub fn default() -> Self {
        let table = [
            31, 0, 1, 2, 3, 4,
            3, 4, 5, 6, 7, 8,
            7, 8, 9, 10, 11, 12,
            11, 12, 13, 14, 15, 16,
            15, 16, 17, 18, 19, 20,
            19, 20, 21, 22, 23, 24,
            23, 24, 25, 26, 27, 28,
            27, 28, 29, 30, 31, 0,
        ];

        ExpansionTable {
            table
        }
    }

    pub fn expand(&self, block: u32) -> u64 {
        let mut result = 0;
        for i in 0..48 {
            let bit = (block >> (32 - self.table[i])) & 1;
            result |= bit << (47 - i);
        }

        result as u64
    }
}

pub struct PermutationTable {
    table: [u8; 32],
}

impl PermutationTable {
    pub fn default() -> Self {
        let table = [
            15, 6, 19, 20, 28, 11, 27, 16,
            0, 14, 22, 25, 4, 17, 30, 9,
            1, 7, 23, 13, 31, 26, 2, 8,
            18, 12, 29, 5, 21, 10, 3, 24,
        ];

        PermutationTable {
            table
        }
    }

    pub fn permute(&self, block: u64) -> u64 {
        let mut result = 0;
        for i in 0..64 {
            let bit = (block >> (63 - self.table[i])) & 1;
            result |= bit << (63 - i);
        }

        result
    }
}

