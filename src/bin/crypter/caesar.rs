
use std::iter::Skip;
use std::env::Args;
use std::fs::File;
use std::io::{stdin, stdout};
use crypter::caesar;

pub fn index(mut args: Skip<Args>) {
    match args.next().expect("caesar requires a `key`. Try \"crypter caesar --help\"").to_lowercase().as_str() {
        "--help" => help(),
        key => {
            let key = key.parse()
                .map_err(|_| ())
                .and_then(|key| if key <= caesar::MAX_KEY {
                    Ok(key)
                } else {
                    Err(())
                }).expect("Key was not a number from [0, 25]. Try \"crypter caesar --help\"");
            
            match args.next().expect("Caesar expected an <option>. Try \"crypter caesar --help\"").to_lowercase().as_str() {
                "-e" | "--encrypt" => match args.next() {
                    None => {
                        println!("Now encrypting each line of input.");
                        caesar::encrypt(key, stdin(), &mut stdout())
                            .unwrap_or_else(|e| eprintln!("Failed while encrypting. {}", e));
                    },
                    Some(in_file) => File::open(in_file)
                        .and_then(|in_file| caesar::encrypt(key, in_file, &mut stdout()))
                        .unwrap_or_else(|e| eprintln!("Failed while encrypting. {}", e))
                },
                "-d" | "--decrypt" => match args.next() {
                    None => {
                        println!("Now encrypting each line of input.");
                        caesar::decrypt(key, stdin(), &mut stdout())
                            .unwrap_or_else(|e| eprintln!("Failed while decrypting. {}", e));
                    },
                    Some(in_file) => File::open(in_file)
                        .and_then(|in_file| caesar::decrypt(key, in_file, &mut stdout()))
                        .unwrap_or_else(|e| eprintln!("Failed while decrypting. {}", e))
                },
                op => eprintln!("Unexpected option {}. Try \"crypter caesar --help\"", op)
            }
        }
    }
}

fn help() {
    eprintln!("Usage: crypter caesar [--help | <option> [in_file]]
    Using --help will display this help message.
    
    <option> is one of:
    -e | --encrypt --- Encrypt the input bytes.
    -d | --decrypt --- Decrypt the input bytes.
    
    [in_file] --- An optional input file to encrypt instead of using the standard input."
    )
}