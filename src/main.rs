use std::io;
use std::env;
extern crate sodiumoxide;
use std::fs::File;
use std::io::{Read, Write};
use sodiumoxide::crypto::secretbox;
// use sodiumoxide::crypto::pwhash;

    /* TODO
    - ask if user is decrypting or encrypting a file
    - ask for a file path
    - open a file
    - pass the file and key to an encryption library
    - encrypt or encrypt the file
    - if encrypting save the encryption key to a different file
    - save and close all files.
    */

const OUTPUT_PATH: &str = "";

fn main() {
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
            // read_from_file(&file_path);
            encrypt_file(&file_path, OUTPUT_PATH);

        } else { // decrypt
            println!("Please enter the path to the file you would like to decrypt: ");
            file_path = console_input_string();
            // decrypt_file();
        }

    } else { 
        // use std in args
        // change args[] to owned instead of immutable reference
        //      may be unnecessary.
        file_path = args[1].to_owned();
        action = args[2].parse::<u32>().expect("error"); // encrypt(1) / decrypt(2)
        
        if action == 1 { // encrypt
            encrypt_file(&file_path, OUTPUT_PATH);

        } else{ // decrypt
            
        }
        // println!("{}, {}", file_path, action);

    }

}

fn read_from_file(file_path: &str) -> io::Result<()>{
    // sorced from https://doc.rust-lang.org/std/fs/struct.File.html
    //let mut file = File::open(file_path)?; // TODO again hard coded file path temp
    let mut file = File::open("testing.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

fn encrypt_file(file_path: &str, output_file_path: &str) {
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();

    let contents = read_from_file(file_path); // TODO can not pass in a file path....
    let mut contents = Vec::new();

    // encrypt file
    // file.read_to_end(&mut contents).unwrap();
    let ciphertext = secretbox::seal(&contents, &nonce, &key);
    
    // Write to file
    // let encrypted_file = File::create(output_file_path); //TODO can not pass in file pat...
    let encrypted_file = File::create("sealed.txt"); // TODO path works if handed as string???
    
    match encrypted_file {
        Ok(mut encrypted_file) => {
            encrypted_file.write_all(&nonce.as_ref()).unwrap();
            encrypted_file.write_all(&ciphertext).unwrap();
            println!("Sucsses!");
        }

        Err(error) => {
            println!("error {}", error);
        }
    }

    // Salt password
    // let password = pwhash::pwhash_simple("your_password", pwhash::OPSLIMIT_INTERACTIVE, pwhash::MEMLIMIT_INTERACTIVE).unwrap();
    // let salted_password = pwhash::gen_salt();
    // let encrypted_key = pwhash::encrypt(&password, &salted_password).unwr

}

/* TODO wating to finish implementation until encrypt works
fn decrypt_file() {
    let mut encrypted_file = File::open(ciphertext_path).map_err(|e| format!("Error opening file: {}", e))?;
    let mut ciphertext = Vec::new();
    encrypted_file.read_to_end(&mut ciphertext).map_err(|e| format!("Error reading file: {}", e))?;

    // Extract the nonce (first bytes) and ciphertext (remaining bytes)
    let nonce_slice = &ciphertext[..secretbox::NONCEBYTES];
    let ciphertext_slice = &ciphertext[secretbox::NONCEBYTES..];

    if nonce_slice
        .len() != secretbox::NONCEBYTES || ciphertext_slice.len() == 0 {
        return Err(String::from:IZZnvalid file format"));
    }

    match secretbox::open(ciphertext_slice, nonce_slice, key) {
        Ok(decrypted_content) => {
            let mut output_file = File::create(output_path).map_err(|e| format!("Error creating output file: {}", e))?;
            output_file.write_all(&decrypted_content).map_err(|e| format!("Error writing decrypted content: {}", e))?;
        }
        Err(e) => Err(format!("Decryption error: {}", e)),
    }
}
*/

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
