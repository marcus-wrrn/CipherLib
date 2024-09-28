
pub struct SBox {
    value: [[u8; 16]; 4],
}

impl SBox {
    pub fn new() -> Self {
        SBox {
            value: [
                [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
                [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
                [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
                [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
            ],
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

}
// impl SBox {
//     fn new() -> Self {
//         SBox {
//             s_boxes: vec![
//                 vec![
//                     vec![1, 0, 3, 2],
//                     vec![3, 2, 1, 0],
//                     vec![0, 2, 1, 3],
//                     vec![3, 1, 3, 2],
//                 ],
//                 vec![
//                     vec![0, 1, 2, 3],
//                     vec![2, 0, 1, 3],
//                     vec![3, 0, 1, 0],
//                     vec![2, 1, 0, 3],
//                 ],
//                 // Add more S-boxes as needed
//             ],
//         }
//     }

//     fn get_value(&self, box_index: usize, row: usize, col: usize) -> u8 {
//         self.s_boxes[box_index][row][col]
//     }
// }



// impl SBox {
//     fn new() -> Self {
//         SBox {
//             s0: [
//                 [1, 0, 3, 2],
//                 [3, 2, 1, 0],
//                 [0, 2, 1, 3],
//                 [3, 1, 3, 2],
//             ],
//             s1: [
//                 [0, 1, 2, 3],
//                 [2, 0, 1, 3],
//                 [3, 0, 1, 0],
//                 [2, 1, 0, 3],
//             ],
//         }
//     }

//     fn get_s0(&self, row: u8, col: u8) -> u8 {
//         self.s0[row as usize][col as usize]
//     }
// }