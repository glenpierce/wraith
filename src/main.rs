//Steganography is the practice of concealing a file, message, image, or video within another file, message, image, or video. The word steganography combines the Greek words steganos (στεγανός), meaning "covered, concealed, or protected", and graphein (γράφειν) meaning "writing".

use std::env;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let fileName: &String = &args[1];
    println!("{}", fileName);

    let mut data:Vec<u8> = Vec::new();
//    let mut data = String::new();
    let file = File::open(fileName.as_str()).expect("Unable to open file");

    let mut bufReader = BufReader::new(file);
    bufReader.read_to_end(&mut data).expect("Unable to read string");
    println!("{:?}", data);

//    let newFile = File::create("encrypted").expect("Unable to create file");
//    let mut newFile = BufWriter::new(newFile);
//    newFile.write_all(data.as_bytes()).expect("Unable to write data");
}
