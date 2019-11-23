//Steganography is the practice of concealing a file, message, image, or video within another file, message, image, or video. The word steganography combines the Greek words steganos (στεγανός), meaning "covered, concealed, or protected", and graphein (γράφειν) meaning "writing".

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

    for byte in image_data {
        remove_least_significant_bit(&byte);
    }

    println!("{:?}", image_data);

    fn remove_least_significant_bit(mut input: &u8) {
        let mask:u8 = 0b1111_1110;
        input = input & mask;
    }

}
