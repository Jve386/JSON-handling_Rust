use std::error::Error; // 
use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{ // Doc: https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about
    let mut reader = csv::Reader::from_path(path)?; // ? is used to return the error if there is one

    for result in reader.records(){ // For each record in the reader
        let record = result?; // ? is used to return the error if there is one (same as above...)
        println!("{:?}", record); 
    }
    Ok(()) // Return Ok if there is no error
} 

fn main(){
    if let Err(e) = read_from_file("./sample.csv"){  // If there is an error, print it
        eprintln!("{}", e);  // Print the error. Documentation: https://doc.rust-lang.org/std/macro.eprintln.html
    }
}