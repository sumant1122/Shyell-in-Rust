use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        // Print the shell prompt
        print!("> ");
        stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Split the input into command and arguments
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        // Match the command
        match command {
            // Change directory command
            "cd" => {
                // Get the new directory, default to "/" if not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                // Change the current directory
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            // Exit command
            "exit" => return,
            // External command
            command => {
                // Spawn a new process for the command
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                // Wait for the command to finish and handle errors
                match child {
                    Ok(mut child) => { child.wait().unwrap(); },
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
