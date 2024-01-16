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

    println!("Would you like to encrypt(1) or decrypt(2) a file: ");
    let action = console_input_int(); // true will return int
    let file_path;
        
    if action == 1 { //encrypt
        println!("Please enter the path to the file you would like to encrypt: ");
        file_path = console_input_string();

    } else { // decrypt
        println!("Please enter the path to the file you would like to decrypt: ");
        file_path = console_input_string();

    }
}

fn file_open(file_path=String){
    

}

fn console_input_int() -> u32{
    // fn parses intager input from the terminal.

    // I got some of this code from:
    //      https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0
    //      however I put it into a function and modifyied it to work 
    //      with strings or ints and just return the i if valid I also added the enum.
    
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

    return user_choice;
}

fn console_input_string() -> String{
    // fn parses string input from the terminal.

    // I got some of this code from:
    //      https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0
    //      however I put it into a function and modifyied it to work 
    //      with strings or ints and just return the i if valid I also added the enum.
    
    // take user input as a string
    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    return input_text;
}