//Steganography is the practice of concealing a file, message, image, or video within another file, message, image, or video. The word steganography combines the Greek words steganos (στεγανός), meaning "covered, concealed, or protected", and graphein (γράφειν) meaning "writing".

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let fileName: &String = &args[1];

//    let mut data:Vec<u32> = Vec::new();
    let mut data = String::new();
    let file = File::open(fileName.as_str()).expect("Unable to open file");

    let mut bufReader = BufReader::new(file);
    bufReader.read_to_string(&mut data).expect("Unable to read string");
    println!("{}", data);
}
