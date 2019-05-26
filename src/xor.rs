//! The `xor` module defines the XOR Cypher encryption algorithm.
//!
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/01/10

use std::iter::Iterator;
use std::io::{Write, Result};

/// Applies the XOR Cypher to the input.
///
/// # Params
/// key_is --- The input stream of byte pairs to encrypt.  
/// os --- The writer to output the encrypted bytes to.
pub fn encrypt(key_is: impl Iterator<Item = (u8, u8)>, os: &mut Write) -> Result<()> {
    //The temporary buffer.
    let mut buf = [0];
    
    for (key, byte) in key_is {
        if byte != b'\n' && byte != b'\r' {
            //Encrypt the byte pair.
            buf[0] = byte ^ key;
            
            if buf[0] == b'\n' || buf[0] == b'\r' {
                //If the byte maps to a line end, revert it.
                buf[0] = byte;
            }
        } else {
            //Leave the byte as is.
            buf[0] = byte;
        }
        
        //The buffer is full, write the bytes out.
        os.write_all(&buf)?;
    }
    
    Ok(())
}

/// This function is provided for consistency but simply calls [`encrypt`](fn.encrypt.html).
///
/// # Params
/// key_is --- The input stream of byte pairs to encrypt.  
/// os --- The writer to output the encrypted bytes to.
pub fn decrypt(key_is: impl Iterator<Item = (u8, u8)>, os: &mut Write) -> Result<()> {
    encrypt(key_is, os)
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;
    
    const PLAIN: &[u8] = b"The red fox, jumped, over the lazy dog! 0123456789";
    const CYPHER: &str = "54696723766062276e66782d2269716876626c25206e74667625726f6d296c60787a24616960292930303030303030303030";
    const KEY: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    #[test]
    fn test_encrypt() {
        let mut cypher = Vec::with_capacity(PLAIN.len());
        
        ::xor::encrypt(repeat(0).zip(PLAIN.iter().map(Clone::clone)), &mut cypher)
            .expect("Failed to encrypt plain-text.");
        assert_eq!(cypher, PLAIN, "Incorrect cypher-text.");
        
        cypher.clear();
        ::xor::encrypt(KEY.iter().cycle().map(Clone::clone).zip(PLAIN.iter().map(Clone::clone)), &mut cypher)
            .expect("Failed to encrypt plain-text.");
        assert_eq!(::hex::encode(cypher), CYPHER, "Incorrect cypher-text.");
    }
    
    #[test]
    fn test_decrypt() {
        let mut plain = Vec::with_capacity(PLAIN.len());
        
        ::xor::decrypt(repeat(0).zip(PLAIN.iter().map(Clone::clone)), &mut plain)
            .expect("Failed to decrypt plain-text.");
        assert_eq!(plain, PLAIN, "Incorrect plain-text.");
        
        plain.clear();
        ::xor::decrypt(KEY.iter().cycle().map(Clone::clone).zip(::hex::decode(CYPHER).unwrap().iter().map(Clone::clone)), &mut plain)
            .expect("Failed to decrypt plain-text.");
        assert_eq!(plain, PLAIN, "Incorrect plain-text.");
    }
}
