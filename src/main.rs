use std::io::{self, Write};
use inquire::{Select, error::InquireError};

use cyphers::vigenere::vigenere;
use cyphers::caesar::caesar;

mod cyphers;


/// ## Used for setting the mode of crypt functions.
#[derive(PartialEq)]
pub enum Mode {
    Encrypt = 0,
    Decrypt = 1,
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

fn run_vigenere () {

    let action = loop {
        let action = vec!["Encrypt", "Decrypt"];
        let option: Result<&str, InquireError> = Select::new("What do you want to do?", action).prompt();

        match option {
            Ok(choice) => match choice {
                "Encrypt" => { break Mode::Encrypt; }
                "Decrypt" => { break Mode::Decrypt; }
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
    };

    print!("\n");

    print!("text: ");
    let text  = loop {
        if let Err(e) = io::stdout().flush() {
            eprintln!("\nI/O error: {}\n", e);
            continue;
        }

        let mut input  = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("\nI/O error: {}\n", e);
            continue;
        }
        let input = input.trim();

        if !input.is_empty() && input.chars().all(|c| c.is_ascii_alphabetic()) {
            break String::from(input);
        } else {
            println!("\nVigenere cypher only works with English alphabet, please try again.\n");
            continue;
        }
    };


    print!("key: ");
    let key = loop {
        if let Err(e) = io::stdout().flush() {
            eprintln!("\nI/O error: {}\n", e);
            continue;
        }

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("I/O error: {}", e);
            continue;
        }
        let input = input.trim();

        if !input.is_empty() && input.chars().all(|c| c.is_ascii_alphabetic()) {
            break String::from(input);
        } else {
            println!("\nVigenere cypher only works with English alphabet, please try again.\n");
            continue;
        }
    };

    print!("\n");

    println!(   
        "{} text: {}",
        if Mode::Encrypt == action { "Encrypted" }
        else                       { "Decrypted" },
        vigenere(&text, &key, action)
    );

    print!("\n");
}

fn run_caesar () {

    let action = loop {
        let action = vec!["Encrypt", "Decrypt"];
        let option: Result<&str, InquireError> = Select::new("What do you want to do?", action).prompt();

        match option {
            Ok(choice) => match choice {
                "Encrypt" => { break Mode::Encrypt; }
                "Decrypt" => { break Mode::Decrypt; }
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
    };

    print!("\n");

    print!("text: ");
    let text  = loop {
        if let Err(e) = io::stdout().flush() {
            eprintln!("\nI/O error: {}\n", e);
            continue;
        }

        let mut input  = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("\nI/O error: {}\n", e);
            continue;
        }
        let input = input.trim();

        if !input.is_empty() && input.chars().all(|c| c.is_ascii_alphabetic()) {
            break String::from(input);
        } else {
            println!("\nCaesar cypher only works with English alphabet, please try again.\n");
            continue;
        }
    };


    print!("key: ");
    let key = loop {
        if let Err(e) = io::stdout().flush() {
            eprintln!("\nI/O error: {}\n", e);
            continue;
        }

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("I/O error: {}", e);
            continue;
        }
        let input: u8 = match input.trim().parse() {
            Ok(num) => { num }
            Err(_) => { 
                println!("key needs to be between 0-255");
                continue;
            } 
        };

        break input;
    };

    print!("\n");

    println!(   
        "{} text: {}",
        if Mode::Encrypt == action { "Encrypted" }
        else                       { "Decrypted" },
        caesar(&text, key, action)
    );

    print!("\n");
}