use std::fs;
use std::io;

fn main() {
    std::process::exit(real_main())
}

fn real_main() -> i32{
    let args: Vec<_> = std::env::args().collect();

    if args.len() <2 { // Logic: we need cargo run + name of the files
        println!("Usage: zip-extract <filename>", args[0]);
        return 1;
    }
}