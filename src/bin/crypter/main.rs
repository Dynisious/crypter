
extern crate crypter;

use std::iter::Skip;
use std::env::{args, Args};

mod caesar;

fn main() {
    let mut args = args().skip(1);
    match args.next() {
        None => help(),
        Some(command) => {
            let command = command.to_lowercase();
            
            if command == "help" {
                help()
            } else {
                index(command, args)
            }
        }
    }
}

fn help() {
    eprintln!("A command-line interface for the `crypter` Rust crate. Prints to standard output.

    crypter <sub-command>

<sub-command>
    help --- Prints this help String.
    caesar --- The Caesar Shift cypher.

For additional help try:

    crypter <sub-command> --help"
    )
}

fn index(command: String, args: Skip<Args>) {
    match command.as_str() {
        "caesar" => caesar::index(args),
        _ => eprintln!("Command {} not recognised, try:
    crypter help", command
        )
    }
}
