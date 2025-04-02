// Exmaple: cargo run 9d0368896a67d6601447b46e2aa7241b8129faa0a66478174f0546214dfa6195
use instant::Instant;
// Import the necessary libraries for SHA-256 hashing and file operations
// Use cargo build to build the project
// Use cargo run to run the project
// Exampe usage: cargo run f0b139242b497c8a5ac33bf176ad89d57176a044708ede42613983ccb97798c3
use sha2::{Digest, Sha256}; // may need to add crate, so cargo add sha2
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
// Reference: https://docs.rs/sha2/latest/sha2/index.html
// Reference: https://medium.com/@TechSavvyScribe/rust-basic-input-and-output-i-o-a-hands-on-guide-60b751531b7f
// Reference: https://doc.rust-lang.org/std/env/fn.args.html
// Reference: https://doc.rust-lang.org/std/fs/struct.File.html
// Reference: https://doc.rust-lang.org/std/io/struct.BufReader.html

/ Main function where program execution begins
fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided (should be 2)
    if args.len() != 2 {
        // If not, print an error message and exit the program with status code 1
        println!("Invalid arguments!");
        println!(">> {} <sha256sum>", args[0]);
        exit(1);
    }

    let wanted_hash = &args[1];

    // Set the file path for the password list (precomputed hashes)
    // let password_file = "src/rockyou.txt";
    let password_file = "src/10-million-password-list-top.txt"; // 8MB

    let mut attempts = 1;
    let start_time = std::time::Instant::now();

    println!("Attempting to crack: {}!\n", wanted_hash);
    // Open the file containing precomputed password hashes and create a reader object
    let password_list = File::open(password_file).unwrap();
    let reader = BufReader::new(password_list);

    for line in reader.lines() {
        // Unwrap the line to extract its contents
        let line = line.unwrap();

        // Extract the password from the line by trimming whitespace and converting it to bytes
        let password = line.trim().to_owned().into_bytes();

        // Compute the SHA-256 hash of the password using the Sha256::digest method
        let password_hash = format!("{:x}", Sha256::digest(&password));

        // Print a message with the attempt number, original password, and computed hash

        println!(
            "[{}] {} == {}",
            attempts,
            std::str::from_utf8(&password).unwrap(),
            password_hash
        );

        if &password_hash == wanted_hash {
            let end_time = Instant::now();
            let duration = end_time.duration_since(start_time);
            println!(
                "Password hash found after {} attempts! {} hashes to {}!",
                attempts,
                std::str::from_utf8(&password).unwrap(),
                password_hash
            );
            println!("Time taken: {:.2?}", duration.as_secs_f64());
            exit(0);
        }

        attempts += 1;
    }

    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    println!("Password hash not found!");
    println!("Time taken: {:.2?}", duration.as_secs_f64());

    exit(0);
}
