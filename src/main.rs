//Steganography is the practice of concealing a file, message, image, or video within another file, message, image, or video. The word steganography combines the Greek words steganos (στεγανός), meaning "covered, concealed, or protected", and graphein (γράφειν) meaning "writing".

use std::env;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};
//use std::io::{BufReader, Read, BufWriter, Write, Cursor};


//extern crate crypto;
//extern crate rand;
//extern crate aesstream;
//
//use crypto::aessafe::{AesSafe128Encryptor, AesSafe128Decryptor};
//use rand::{Rng, OsRng};
//use aesstream::{AesWriter, AesReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name: &String = &args[1];
    println!("{}", file_name);

    let mut data:Vec<u8> = Vec::new();

    let file = File::open(file_name.as_str()).expect("Unable to open file");

    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_end(&mut data).expect("Unable to read string");
    println!("{:?}", data);

    let new_file = File::create("encrypted").expect("Unable to create file");
    let mut new_file = BufWriter::new(new_file);
    let mut pos = 0;
    while pos < data.len() {
        let bytes_written = new_file.write(&data[pos..]).expect("unable to write to file");
        pos += bytes_written;
    }
}
