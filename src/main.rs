use std::io::{self, Write};
use inquire::{Select, error::InquireError};

use cyphers::vigenere::vigenere;
use cyphers::caesar::caesar;

mod cyphers;


/// ## Used for setting the mode of crypt functions.
#[derive(PartialEq)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

fn main () {
    loop {
        print!("\n\n");
        
        let cyphers = vec!["Vigenere", "Caesar"];
        let option: Result<&str, InquireError> = Select::new("What cypher do you want to use?", cyphers).prompt();

        print!("\n");

        match option {
            Ok(choice) => match choice {
                "Vigenere" => { run_vigenere() }
                "Caesar"   => { run_caesar()   }
                _ => {
                    println!("\nNo match found, please try again\n");
                    continue;
                }   
            },
            Err(_) => { 
                println!("\nExtracting multi choice input failed, please try again.\n");
                continue;
            }
        }

        let quit = vec!["yes", "no"];
        let option: Result<&str, InquireError> = Select::new("Do you want to quit?", quit).prompt();

        match option {
            Ok(choice) => match choice {
                "yes" => break,
                "no" => continue,
                _ => {
                    println!("\nNo match found, please try again\n");
                    continue;
                }   
            },
            Err(_) => { 
                println!("\nExtracting multi choice input failed, please try again.\n");
                continue;
            }
        }
    }
}

/// ## Used for setting the mode of input function.
enum InputMode {
    String,
    AsciiString,
    U8,
}

/// ## Used by input function to return multiple types.
enum Input {
    String(String),
    AsciiString(String),
    U8(u8),
}

/// # Asks user for input, can work with multiple types.
/// ## Panic
/// 
/// #### this function doesn't panic but will ask for input again if it fails or the conditions aren't met.
fn read_input(mode: InputMode, message: &str) -> Input {
    loop {
        print!("{}", message);

        if let Err(e) = io::stdout().flush() {
            eprintln!("\nI/O error flushing stdout: {}\n", e);
            continue;
        }

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("I/O error reading input: {}", e);
            continue;
        }

        let input = input.trim();

        match mode {
            InputMode::String => {
                if !input.is_empty() { return Input::String(input.to_string()) }
                else                 { println!("\nValue can't be empty.\n")   }
            }
            InputMode::U8 => {
                match input.parse::<u8>() {
                    Ok(num) => return Input::U8(num),
                    Err(_)      => println!("\nValue needs to be between 0-255\n"),
                }
            }
            InputMode::AsciiString => {
                if input.is_empty()                                           { println!("\nValue can't be empty.\n") }
                else if !input.chars().all(|c| c.is_ascii_alphabetic()) { println!("\nValue needs to contain only ASCII alphabetic.\n") }
                else                                                          { return Input::AsciiString(input.to_string()) }
            }
        }
    }
}

fn read_mode() -> Mode {
    loop {
        let modes = vec!["Encrypt", "Decrypt"];
        let input: Result<&str, InquireError> = Select::new("What do you want to do?", modes).prompt();

        match input {
            Ok(choice) => match choice {
                "Encrypt" => { return Mode::Encrypt; }
                "Decrypt" => { return Mode::Decrypt; }
                _ => {
                    println!("\nNo match found, please try again\n");
                    continue;
                }   
            },
            Err(_) => { 
                println!("\nExtracting multi choice input failed, please try again.\n");
                continue;
            }
        }   
    }
}

fn run_vigenere () {
    let mode = read_mode();

    print!("\n");
    let Input::String(text) = read_input(InputMode::String, "text: ") else {
        unreachable!()
    };

    let Input::AsciiString(key) = read_input(InputMode::AsciiString, "key: ") else {
        unreachable!()
    };

    println!(   
        "\n{} text: {}\n",
        if Mode::Encrypt == mode { "Encrypted" }
        else                     { "Decrypted" },
        vigenere(&text, &key, mode)
    );
}

fn run_caesar () {

    let mode = read_mode();

    print!("\n");
    let Input::String(text) = read_input(InputMode::String, "text: ") else {
        unreachable!()
    };

    let Input::U8(key) = read_input(InputMode::U8, "key: ") else {
        unreachable!()
    };

    println!(   
        "\n{} text: {}\n",
        if Mode::Encrypt == mode { "Encrypted" }
        else                     { "Decrypted" },
        caesar(&text, key, mode)
    );
}