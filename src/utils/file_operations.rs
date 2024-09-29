use std::fs::File;
use std::io::{self, Write, Read};
use crate::ciphers::block_ciphers::sdes;

pub fn save_substitution_boxes_to_file(sub_boxes: sdes::SBoxes, file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;

    let num_boxes = sub_boxes.s_boxes.len() as u8;
    let row_size = sub_boxes.s_boxes[0].value().len() as u8;

    file.write_all(&[num_boxes])?;
    file.write_all(&[row_size])?;

    for sbox in sub_boxes.s_boxes.iter() {
        for row in sbox.value().iter() {
            file.write_all(row)?;
        }
    }
    
    Ok(())
}

pub fn read_substitution_boxes(file_name: &str) -> io::Result<sdes::SBoxes> {
    let mut file = File::open(file_name)?;
    let mut buffer: [u8; 2] = [0; 2];
    file.read_exact(&mut buffer)?;

    let num_boxes = buffer[0] as usize;
    let row_size = buffer[1] as usize;

    let mut buffer = vec![0; num_boxes * row_size * 16];

    file.read_exact(&mut buffer)?;

    let mut s_boxes = Vec::new();
    for block in buffer.chunks(row_size * 16) {
        let mut s_box = [[0; 16]; 4];
        for (i, row) in block.chunks(16).enumerate() {
            for (j, value) in row.iter().enumerate() {
                s_box[i][j] = *value;
            }            
        }
        s_boxes.push(sdes::SBox::from(s_box));
    }

    Ok(sdes::SBoxes::from(s_boxes))
}