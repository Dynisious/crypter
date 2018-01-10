
use std::iter::Skip;
use std::env::Args;
use std::fs::File;
use std::io::{Read, stdin, stdout};
use crypter::caesar;

pub fn index(mut args: Skip<Args>) {
    let key = match args.next() {
        Some(arg) => match arg.as_str() {
            "--help" => return help(),
            key => match key.parse()
                .map_err(|_| ())
                .and_then(|key| if key <= caesar::MAX_KEY {
                    Ok(key)
                } else {
                    Err(())
                }) {
                Ok(key) => key,
                Err(_) => return eprintln!("Key was not a number from [0, 25]. Try: crypter caesar --help")
            }
        },
        None => return eprintln!("caesar requires a `key`. Try: crypter caesar --help")
    };
    
    match args.next() {
        Some(arg) => match arg.as_str() {
            "-e" | "--encrypt" => match args.next() {
                None => {
                    eprintln!("Now encrypting each line of input.");
                    caesar::encrypt(
                        key,
                        stdin().bytes()
                            .take_while(|byte| byte.is_ok())
                            .map(|byte| byte.unwrap()),
                        &mut stdout()
                    ).unwrap_or_else(|e| eprintln!("Failed while encrypting. {}", e));
                },
                Some(in_file) => File::open(in_file)
                    .and_then(|in_file| caesar::encrypt(
                        key,
                        in_file.bytes()
                            .take_while(Result::<u8, _>::is_ok)
                            .map(Result::<u8, _>::unwrap),
                        &mut stdout()
                    )).unwrap_or_else(|e| eprintln!("Failed while encrypting. {}", e))
            },
            "-d" | "--decrypt" => match args.next() {
                None => {
                    eprintln!("Now encrypting each line of input.");
                    caesar::decrypt(
                        key,
                        stdin().bytes()
                            .take_while(Result::<u8, _>::is_ok)
                            .map(Result::<u8, _>::unwrap),
                        &mut stdout()
                    ).unwrap_or_else(|e| eprintln!("Failed while decrypting. {}", e));
                },
                Some(in_file) => File::open(in_file)
                    .and_then(|in_file| caesar::decrypt(
                        key,
                        in_file.bytes()
                            .take_while(Result::<u8, _>::is_ok)
                            .map(Result::<u8, _>::unwrap),
                        &mut stdout())
                    ).unwrap_or_else(|e| eprintln!("Failed while decrypting. {}", e))
            },
            arg => eprintln!("Unexpected option {}. Try: crypter caesar --help", arg)
        },
        None => eprintln!("caesar expected an <option>. Try: crypter caesar --help")
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
