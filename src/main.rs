//Steganography is the practice of concealing a file, message, image, or video within another file, message, image, or video. The word steganography combines the Greek words steganos (στεγανός), meaning "covered, concealed, or protected", and graphein (γράφειν) meaning "writing".

//lossless .png can contain base64 encoded jpeg

use std::env;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};
//use std::io::{BufReader, Read, BufWriter, Write, Cursor};


extern crate crypto;
extern crate rand;
extern crate aesstream;

use crypto::aessafe::{AesSafe128Encryptor, AesSafe128Decryptor};
use aesstream::{AesWriter, AesReader};
use rand::AsByteSliceMut;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name: &String = &args[1];
    println!("{}", file_name);

    let mut data:Vec<u8> = Vec::new();

    let file = File::open(file_name.as_str()).expect("Unable to open file");

    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_end(&mut data).expect("Unable to read string");
    println!("{:?}", data);

    let key:[u8; 16] = [1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6];

    let encryptor = AesSafe128Encryptor::new(&key);
    let mut encrypted = Vec::new();
    {
        let mut writer = AesWriter::new(&mut encrypted, encryptor).expect("writer problem?");
        writer.write_all(data.as_byte_slice_mut()).expect("Unable to write with encryptor");
    }
    println!("{}", "encrypted file data");
    println!("{:?}", encrypted);
//    let mut writer = AesWriter::new(file, encryptor)?;
//    writer.write_all("Hello World!".as_bytes())?;

    let new_file = File::create("encrypted").expect("Unable to create file");
    let mut new_file = BufWriter::new(new_file);
    let mut pos = 0;
    while pos < data.len() {
        let bytes_written = new_file.write(&encrypted[pos..]).expect("unable to write to file");
        pos += bytes_written;
    }

    //open image file
    let image_file_name: &String = &args[2];
    println!("{}", image_file_name);

    let mut image_data:Vec<u8> = Vec::new();

    let image_file = File::open(image_file_name.as_str()).expect("Unable to open file");

    let mut image_buf_reader = BufReader::new(image_file);
    image_buf_reader.read_to_end(&mut image_data).expect("Unable to read string");
    println!("{:?}", image_data);

    for byte in image_data.iter_mut() {
        remove_least_significant_bit(byte);
    }

    println!("{}", "least significant bit removed from image file");
    println!("{:?}", image_data);

    let mut iteration:usize = 0;
    for mut byte in image_data.iter_mut() {
        replace_least_significant_bit(byte, iteration, &encrypted);
        iteration += 1;
    }

    println!("{}", "encrypted data injected");
    println!("{:?}", image_data);

    println!("{}", "least significant bit from new image data");
    let mut newData:Vec<u8> = Vec::new();
    for byte in image_data {
        let nextBit:u8 = get_bit_at(byte, 0);
        newData.push(nextBit);
    }
    println!("{:?}", newData); //AWESOME! The bits are there... they're kind of backwards, but they're THERE!
}

fn remove_least_significant_bit(mut input: &mut u8) {
    let mask:u8 = 0b1111_1110; // let m = !1u8;
    *input = *input & mask;
}

fn replace_least_significant_bit(host: &mut u8, iteration:usize, inserted_bytes:&[u8]) {
    if iteration >= inserted_bytes.len() {
        return;
    }
    let position_in_byte:u8 = (iteration % 8) as u8;
    let position_in_byte_array:usize = (iteration / 8) as usize;
    let bit_to_insert:u8 = get_bit_at(inserted_bytes[position_in_byte_array], position_in_byte);

    if bit_to_insert > 0 {
        *host = bit_twiddling(*host, 0);
    }
}

fn get_bit_at(input: u8, n: u8) -> u8 {
    if n < 8 {
        if input & (1 << n) != 0 {
            1
        } else {
            0
        }
    } else {
        0
    }
}

fn bit_twiddling(original: u8, bit: u8) -> u8 {
    let mask = 1 << bit;
    return original | mask;
}
