use std::io;

fn main() {
    /*
    - ask if user is decrypting or encrypting a file
    - ask for a file path
    - open a file
    - pass the file and key to an encryption library
    - encrypt or encrypt the file
    - if encrypting save the encryption key to a different file
    - save and close all files.
     */

    // I got this code from:
    //      https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0
    println!("Would you like to encrypt(1) or decrypt(2) a file: ");
    
    // take user input as a string
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let mut user_choice: u32 = 0; 

    // parse the string to an int
    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {user_choice = i},
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    if user_choice == 1 {
        println!("ye");
    } else if user_choice == 2 {
        println!("yo");
    }
}