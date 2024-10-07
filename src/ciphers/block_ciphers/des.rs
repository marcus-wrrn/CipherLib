use std::io;
use crate::utils::file_operations;
use crate::utils::bit_operations::{get_bit, get_bit_slice, get_bit_u64};
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

    pub fn substitution(&self, value: u8) -> u8 {
        const BIT_LENGTH: u32 = 6;
        let row = ((get_bit(value as u32, BIT_LENGTH - 1) << 1) + get_bit(value as u32, 0)) as usize;
        let col = get_bit_slice(value as u64, 1, 4) as usize;
        self.value[row][col]
    }
}

pub struct SBoxes {
    pub s_boxes: Vec<SBox>,
}

impl SBoxes {
    pub fn new() -> Self {
        let s1_box: [[u8; 16]; 4] = [
            [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
            [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
            [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
            [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13]
        ];

        let s2_box: [[u8; 16]; 4] = [
            [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
            [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
            [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
            [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
        ];

        let s3_box: [[u8; 16]; 4] = [
            [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
            [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
            [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
            [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
        ];

        let s4_box: [[u8; 16]; 4] = [
            [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
            [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
            [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
            [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
        ];


        let s5_box: [[u8; 16]; 4] = [
            [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
            [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
            [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
            [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
        ];


        let s6_box: [[u8; 16]; 4] = [
            [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
            [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
            [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
            [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
        ];


        let s7_box: [[u8; 16]; 4] = [
            [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
            [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
            [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
            [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
        ];


        let s8_box: [[u8; 16]; 4] = [
            [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
            [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
            [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
            [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
        ];

        SBoxes::from(vec![
            SBox::from(s1_box),
            SBox::from(s2_box),
            SBox::from(s3_box),
            SBox::from(s4_box),
            SBox::from(s5_box),
            SBox::from(s6_box),
            SBox::from(s7_box),
            SBox::from(s8_box),
        ])
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

    pub fn substitution(&self, value: u64) -> u32 {
        //let max_bit = 32;
        let mut result: u32 = 0;
        for i in 0..self.s_boxes.len() {
            let start_value = ((self.s_boxes.len() - 1 - i) * 6) as u8;
            let block = get_bit_slice(value, start_value, start_value + 5) as u8;
            let sub = self.s_boxes[i].substitution(block);
            result |= (sub as u32) << ((7 - i) * 4);
        }
        result
    }
}

pub struct ExpansionTable {
    table: [u8; 48],
}

impl ExpansionTable {
    pub fn new() -> Self {
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

    pub fn from(table: [u8; 48]) -> Self {
        ExpansionTable {
            table
        }
    }

    pub fn expand(&self, block: u32) -> u64 {
        let mut result: u64 = 0;
        for i in 0..self.table.len() {
            let index = self.table[i] as u32;
            let bit = get_bit(block, index);

            result |= (bit as u64) << i;
        }

        result
    }
}

pub struct PermutationTable {
    table: [u8; 32],
}

impl PermutationTable {
    pub fn new() -> Self {
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

    pub fn from(table: [u8; 32]) -> Self {
        PermutationTable {
            table
        }
    }

    pub fn permute(&self, block: u32) -> u32 {
        let mut result = 0;
        const BIT_LENGTH: u8 = 31;
        for i in 0..32 {
            let index = self.table[i] as u8;
            let bit = get_bit(block, (BIT_LENGTH - index) as u32);
            result |= bit << BIT_LENGTH - i as u8;
        }
        result
    }
}

pub struct IPTable {
    table: [u8; 64],
}

impl IPTable {
    pub fn new() -> Self {
        let table: [u8; 64] = [
            57, 49, 41, 33, 25, 17, 9, 1,
            59, 51, 43, 35, 27, 19, 11, 3,
            61, 53, 45, 37, 29, 21, 13, 5,
            63, 55, 47, 39, 31, 23, 15, 7,
            56, 48, 40, 32, 24, 16, 8, 0,
            58, 50, 42, 34, 26, 18, 10, 2,
            60, 52, 44, 36, 28, 20, 12, 4,
            62, 54, 46, 38, 30, 22, 14, 6,
        ];

        IPTable {
            table
        }
    }

    pub fn new_inverse() -> Self {
        let table: [u8; 64] = [
            39, 7, 47, 15, 55, 23, 63, 31,
            38, 6, 46, 14, 54, 22, 62, 30,
            37, 5, 45, 13, 53, 21, 61, 29,
            36, 4, 44, 12, 52, 20, 60, 28,
            35, 3, 43, 11, 51, 19, 59, 27,
            34, 2, 42, 10, 50, 18, 58, 26,
            33, 1, 41, 9, 49, 17, 57, 25,
            32, 0, 40, 8, 48, 16, 56, 24,
        ];

        IPTable {
            table
        }
    }

    pub fn permute(&self, block: u64) -> u64 {
        const BIT_LENGTH: u8 = 63;
        let mut result = 0;
        for i in 0..64 {
            let index = self.table[i];
            let bit = get_bit_u64(block, BIT_LENGTH - index);
            result |= bit << BIT_LENGTH - i as u8;
        }
        result
    }
}

pub struct KeyPermutationTable {
    table: [u8; 56],
}

impl KeyPermutationTable {
    pub fn new() -> Self {
        let table: [u8; 56] = [
            56, 48, 40, 32, 24, 16, 8, 0,
            57, 49, 41, 33, 25, 17, 9, 1,
            58, 50, 42, 34, 26, 18, 10, 2,
            59, 51, 43, 35, 62, 54, 46, 38,
            30, 22, 14, 6, 61, 53, 45, 37,
            29, 21, 13, 5, 60, 52, 44, 36,
            28, 20, 12, 4, 27, 19, 11, 3,
        ];

        KeyPermutationTable {
            table
        }
    }

    pub fn permute(&self, block: u64) -> u64 {
        const BIT_LENGTH: u8 = 55;
        let mut result = 0;
        let row_length = 8;
        let col_length = 7;
        for i in 0..56 {
            let index = self.table[i];
            let bit = get_bit_u64(block, BIT_LENGTH - index);
            let row = i / row_length;
            let col = i % col_length;
            result |= bit << BIT_LENGTH - (row * row_length + col) as u8;
        }
        result
    }
}

pub struct KeyCompressionTable {
    table: [u8; 48],
}

impl KeyCompressionTable {
    pub fn new() -> Self {
        let table: [u8; 48] = [
            13, 16, 10, 23, 0, 4, 2, 27,
            14, 5, 20, 9, 22, 18, 11, 3,
            25, 7, 15, 6, 26, 19, 12, 1,
            40, 51, 30, 36, 46, 54, 29, 39,
            50, 44, 32, 47, 43, 48, 38, 55,
            33, 52, 45, 41, 49, 35, 28, 31,
        ];

        KeyCompressionTable {
            table
        }
    }

    pub fn permute(&self, block: u64) -> u64 {
        const BIT_LENGTH: u8 = 47;
        let row_length = 8;
        let col_length = 6;

        let mut result = 0;
        for i in 0..48 {
            let index = self.table[i];
            let bit = get_bit_u64(block, BIT_LENGTH - index);
            let row = i / row_length;
            let col = i % col_length;
            result |= bit << BIT_LENGTH - (row * row_length + col) as u8;
        }
        result
    }
}

