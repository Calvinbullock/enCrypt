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
    let action = console_input(true);

    let file_path = console_input(false);

    // println!("{}, ", action);
    // println!("{}, ", file_path);
    
}

// allows me to return ether an int or a string from console_input
enum ResultValue{
    IntResult(u32),
    StringResult(String),
}

fn console_input(is_int: bool) -> ResultValue{
    // fn parses input from the terminal.
    //      if you exspect a string set is_int false, if you want an int enter true.

    // I got this code from:
    //      https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0
    //      however I put it into a function and modifyied it to work 
    //      with strings or ints and just return the i if valid.
    
    // take user input as a string
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    if is_int {  
        let mut user_choice: u32 = 0; 
        
        // parse the string to an int
        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => {user_choice = i},
            Err(..) => println!("this was not an integer: {}", trimmed),
        };

        ResultValue::IntResult(user_choice)

    } else {
        ResultValue::StringResult(input_text)

    }

}