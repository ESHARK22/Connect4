// Taken from https://github.com/ESHARK22/AqaGCSEComputerScienceStuff/blob/main/InputHandlers/src/lib.rs

use std::io::{stdin, stdout, Write};

pub fn input(prompt: impl AsRef<str> + std::fmt::Display) -> String {
    loop {
        print!("{prompt}");

        // Flush stdout, since its a new line buffer, but we are not printing a new line
        if let Err(error) = stdout().flush() {
            // Print the error message and go to the next iteration
            println!("error: {error}",);
            println!("Try again...\n");
            continue;
        }

        // Create a buffer, or clear the old one
        let mut inp_buffer = String::new();

        // Read stdin into the "buffer"
        match stdin().read_line(&mut inp_buffer) {
            Ok(_) => {
                return inp_buffer;
            }
            Err(error) => {
                // Print the error message and go to the next iteration
                println!("error: {error}",);
                println!("Try again...\n");
                continue;
            }
        }
    }
}

pub fn int_input(prompt: impl AsRef<str> + std::fmt::Display) -> usize {
    loop {
        // Get the input
        let str_input = input(&prompt);

        // Try and convert that string to an int
        let int_input = str_input.trim().parse::<usize>();

        // If it converted fine, return the int, else ask again
        match int_input {
            Ok(int) => return int,
            Err(error) => {
                println!("error: {error}");
                println!("Try again...");
                continue;
            }
        }
    }
}

pub fn yes_no_input(prompt: impl AsRef<str> + std::fmt::Display) -> bool {
    // Yes -> True
    // No  -> False

    loop {
        // Get the input, remove trailing whitespaces, and make it lowercase
        let str_input = input(&prompt).trim().to_lowercase();

        if str_input.as_str() == "y" || str_input.as_str() == "yes" {
            return true;
        } else if str_input.as_str() == "n" || str_input.as_str() == "no" {
            return false;
        } else {
            // Not a valid input
            println!("Invalid input!");
            println!("Please only enter either yes or no... \n")
        };
    }
}
