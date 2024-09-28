use std::fs::File;
use std::io::{self, Write, Read};
use crate::ciphers::block_ciphers::sdes;

fn save_substitution_boxes_to_file(sub_boxes: sdes::SBoxes, file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;

    for sbox in sub_boxes.s_boxes.iter() {
        for row in sbox.value().iter() {
            file.write_all(row)?;
        }
    }
    
    Ok(())
}

fn read_substitution_boxes(file_name: &str, num_boxes: usize, row_size: usize) -> io::Result<sdes::SBoxes> {
    let mut file = File::open(file_name)?;
    let mut buffer = vec![0; num_boxes * row_size];

    file.read_exact(&mut buffer)?;

    let mut s_boxes = Vec::new();
    for block in buffer.chunks(row_size) {
        let mut s_box = [[0; 16]; 4];
        for (i, row) in block.chunks(4).enumerate() {
            for (j, value) in row.iter().enumerate() {
                s_box[i][j] = *value;
            }            
        }
        s_boxes.push(sdes::SBox::from(s_box));
    }

    Ok(sdes::SBoxes::from(s_boxes))
}