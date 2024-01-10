use std::fs;
use std::io;

fn main() {
    std::process::exit(real_main())
}

// Reason why we use real_main.
// How to cleanly end the program with an exit code; similar to return exit_code in C:
// https://stackoverflow.com/questions/30281235/how-to-cleanly-end-the-program-with-an-exit-code
fn real_main() -> i32{
    let args: Vec<_> = std::env::args().collect();

    if args.len() <2 { // Check if the user has provided a filename
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let fname = std::path::Path::new(&*args[1]); // Convert the string to a path
    let file = fs::File::open(&fname).unwrap(); // Open the file
    let mut archive = zip::ZipArchive::new(file).unwrap(); // Create a zip archive (mutable)

    for i in 0..archive.len() { // Iterate over the files in the archive
        let mut file = archive.by_index(i).unwrap(); // Get the files (1 by 1)

        // It is dangerous to extract files without validation; reason why we use the next method:
        // (https://docs.rs/zip/0.5.13/zip/read/struct.ZipFile.html#ZipFile::enclosed_name)
        let outpath = match file.enclosed_name() { // method to validate the name as a safe path
            Some(path) => path.to_owned(), // If the path is safe, return the path
            None => continue, // If the path is not safe, continue to the next file
        };
        {
            let comment = file.comment(); // Get the comment of the file
            if !comment.is_empty(){ // If the comment is not empty
                println!("File {} comment: {}", i, comment); // Print the comment
            }
        }
        if (*file.name()).ends_with('/'){ // If the file is a directory
            println!("File {} extracted to \"{}\"", i, outpath.display()); // Print the file name
            fs::create_dir_all(&outpath).unwrap(); // Create the directory: (Function explanation https://doc.rust-lang.org/std/fs/fn.create_dir_all.html)
        } else{ // If the file is not a directory
            println!(
                "File {} extracted to \"{}\" ({} bytes)", // Print the file name and size
                i,
                outpath.display(),
                file.size() // (Function explanation https://docs.rs/zip/0.5.13/zip/read/struct.ZipFile.html#ZipFile::size)
            );
            if let Some(p) = outpath.parent(){ // If the file has a parent
                if !p.exists(){ // If the parent does not exist
                    fs::create_dir_all(&p).unwrap(); // Create the parent directory (Function explanation https://doc.rust-lang.org/std/fs/fn.create_dir_all.html)
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap(); // Create the file
            io::copy(&mut file, &mut outfile).unwrap(); // Copy the file
        }
        #[cfg(unix)] // If the OS is unix
        {
            use std::os::unix::fs::PermissionsExt; // Import the permissions extension
            if let Some(mode) = file.unix_mode(){ // If the file has a unix mode
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap(); // Set the permissions of the file
            }
        }
    }
    0   // Return 0 to indicate that the program has ended successfully
}