
use std::iter::{Skip, repeat};
use std::env::Args;
use std::fs::File;
use std::io::{Read, stdin};
use crypter::xor;

pub fn index(mut args: Skip<Args>) {
    let key = match args.next() {
        Some(arg) => match arg.as_str() {
            "--help" => return help(),
            "-k" | "--key" => match args.next() {
                Some(key) => match ::hex::decode(key) {
                    Ok(key) => Ok(key),
                    Err(e) => return eprintln!("Error decoding `key`. {}", e)
                },
                None => return eprintln!("Expected a hexidecimal key argument to `--key`. Try: crypter xor --help")
            },
            "--key-file" => match args.next() {
                Some(key_file) => Err(key_file),
                None => return eprintln!("Expected a file path argument to `--key-file`. Try: crypter xor --help")
            },
            _ => return eprintln!("Expected one of `--key` or `--key-file`. Try: crypter xor --help")
        },
        None => return eprintln!("Expected one of `--key` or `--key-file`. Try: crypter xor --help")
    };
    let key = match key.as_ref() {
        Ok(key) => Ok(key.iter().cycle().map(Clone::clone)),
        Err(key_file) => match File::open(key_file) {
            Ok(key_file) => Err(key_file.bytes().filter_map(Result::ok)),
            Err(e) => return eprintln!("Error opening key-file. {}", e)
        }
    };
    
    match args.next() {
        Some(in_file) => match File::open(in_file) {
            Ok(in_file) => {
                let is = repeat(()).scan(
                    in_file.bytes().filter_map(Result::ok).filter(u8::is_ascii_hexdigit),
                    |bytes, _| bytes.next()
                        .and_then(|head| bytes.next()
                            .and_then(|tail| ::hex::decode(&[head, tail]).ok()
                                .map(|byte| byte[0])))
                );
                match key {
                    Ok(key_bytes) => xor::encrypt(key_bytes.zip(is), &mut ::hexout::stdhex())
                        .unwrap_or_else(|e| eprintln!("Failed while encrypting. {}", e)),
                    Err(key_bytes) => xor::encrypt(key_bytes.zip(is), &mut ::hexout::stdhex())
                        .unwrap_or_else(|e| eprintln!("Failed while encrypting. {}", e))
                }
            },
            Err(e) => return eprintln!("Error opening in-file. {}", e)
        },
        None => {
            eprintln!("Now encrypting each line of input.");
            let is = repeat(()).scan(
                stdin().bytes().filter_map(Result::ok).filter(u8::is_ascii_hexdigit),
                |bytes, _| bytes.next()
                    .and_then(|head| bytes.next()
                        .and_then(|tail| ::hex::decode(&[head, tail]).ok()
                            .map(|byte| byte[0])))
            );
            match key {
                Ok(key_bytes) => xor::encrypt(key_bytes.zip(is), &mut ::hexout::stdhex())
                        .unwrap_or_else(|e| eprintln!("Failed while encrypting. {}", e)),
                Err(key_bytes) => xor::encrypt(key_bytes.zip(is), &mut ::hexout::stdhex())
                    .unwrap_or_else(|e| eprintln!("Failed while encrypting. {}", e))
            }
        }
    }
}

fn help() {
    unimplemented!()
}
