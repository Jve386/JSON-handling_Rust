extern crate flate2; // Import the flate2 crate

use flate2::write::GzEncoder; // Import the GzEncoder
use flate2::Compression; // Import the Compression enum
use std::env::args; // Import the args function from the env module
use std::fs::File; // Import the File struct from the fs module
use std::io::copy; // Import the copy function from the io module
use std::io::BufReader; // Import the BufReader struct from the io module
use std::time::Instant; // Import the Instant struct from the time module (How much time has passed since the program started)


fn main() {
    if args().len() !=3 { // Check if the user has provided the correct number of arguments
        eprintln!("Usage: `source` `target`"); // Print the usage
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap()); // Open the file and create a BufReader
    let output = File::create(args().nth(2).unwrap()).unwrap(); // Create the output file
    let mut encoder = GzEncoder::new(output, Compression::default()); // Create the encoder
    let start = Instant::now(); // Start the timer
    copy(&mut input, &mut encoder).unwrap(); // Copy the input to the encoder
    let output = encoder.finish().unwrap(); // Finish the encoder
    println!(
        "Source len: {:?}", // Print the length of the source
        input.get_ref().metadata().unwrap().len() 
    );
    println!(
        "Target len:{:?}", 
        output.metadata().unwrap().len() 
    );
    println!(
        "Time elapsed: {:?}", 
        start.elapsed() 
    );
}