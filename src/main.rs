use std::io;
use std::env;
extern crate sodiumoxide;
// use sodiumoxide;

fn main() {
    /* TODO
    - ask if user is decrypting or encrypting a file
    - ask for a file path
    - open a file
    - pass the file and key to an encryption library
    - encrypt or encrypt the file
    - if encrypting save the encryption key to a different file
    - save and close all files.
    */

    let file_path; 
    let action; 
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // use normal console in / out 
        // TODO move this all to a function the returns action
        
        println!("Would you like to encrypt(1) or decrypt(2) a file: ");
        let temp = console_input_string();
        action = string_to_int(temp); // true will return int
        
        if action == 1 { //encrypt
            println!("Please enter the path to the file you would like to encrypt: ");
            file_path = console_input_string();
            // salty_file_encryption(file_path);

        } else { // decrypt
            println!("Please enter the path to the file you would like to decrypt: ");
            file_path = console_input_string();

        }
        println!("{file_path}");

    } else { 
        // use std in args
        // change args[] to owned instead of immutable reference
        //      may be unnecessary..
        file_path = args[1].to_owned();
        action = args[2].parse::<u32>().expect("error"); // encrypt(1) / decrypt(2)

        println!("{}, {}", file_path, action);

    }

}

fn salty_file_encryption(file_path=String) {
    use sodiumoxide::crypto::secretbox;
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    let plaintext = b"some data";
    let ciphertext = secretbox::seal(plaintext, &nonce, &key);
    let their_plaintext = secretbox::open(&ciphertext, &nonce, &key).unwrap();
    assert!(plaintext == &their_plaintext[..]);
}

fn string_to_int(string_in: String) -> u32{ 
    // fn parses string input to int.

    // I got some of this code from:
    //      https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0
    //      I separated this part into it's own function for more flexibility.

    let mut user_choice: u32 = 0;
    let trimmed = string_in.trim();

    match trimmed.parse::<u32>() {
        Ok(i) => {user_choice = i},
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    return user_choice;
}

fn console_input_string() -> String{
    // fn parses string input from the terminal.

    // I got some of this code from:
    //      https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0
    //      however I put it into a function and modified it to pass the i value out to the return value.
    
    // take user input as a string
    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    return input_text;
}