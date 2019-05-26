//! The `caesar` module defines the Caesar Shift encryption algorithm.
//!
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/01/09

use std::iter::Iterator;
use std::io::{Write, Result, Error, ErrorKind};

/// The length of the alphabet being encrypted (`A-Z`).
const ALPHA_LEN: u8 = 26;
/// The maximum `key` value for this cypher.
pub const MAX_KEY: u8 = 25;

/// Applies the Caesar Shift cypher to the input.
///
/// Only characters from the range `a-z` or `A-Z` will be affected.  
/// Input values of `a-z` will be converted to `A-Z` before encryption.
///
/// # Params
/// key --- A value from `[0, 26)` which specifies the shift to apply.  
/// is --- The input stream of bytes to encrypt.  
/// os --- The writer to output the encrypted bytes to.
///
/// # Errors
///
/// An error of "Bad Key." will be returned if `key` is not in the range `[0, 26)`.
pub fn encrypt(key: u8, is: impl Iterator<Item = u8>, os: &mut Write) -> Result<()> {
    if key < ALPHA_LEN {
        //The key is a valid shift.
        //The temporary buffer.
        let mut buf = [0];
        let mut encrypt = |byte: u8| {
            //Place the byte in the temporary buffer.
            buf[0] = byte;
            
            //If the byte is ascii-alphabetic, it gets encrypted.
            if buf[0].is_ascii_alphabetic() {
                //Shift the byte.
                buf[0] = buf[0].to_ascii_uppercase() + key;
                
                if buf[0] > b'Z' {
                    //The byte was pushed out of the ascii-alphabetic range.
                    //Wrap the byte around to `A`.
                    buf[0] -= ALPHA_LEN;
                }
            }
            
            //Write the byte out.
            os.write_all(&buf)
        };
        
        for byte in is {
            //Encrypt the byte.
            encrypt(byte)?;
        }
        
        Ok(())
    } else {
        //The key is not a valid shift.
        Err(Error::new(ErrorKind::Other, "Bad Key."))
    }
}

/// The inverse of [`encrypt`](fn.encrypt.html).
///
/// # Params
/// key --- A value from `[0, 26)` which specifies the shift to reverse.  
/// is --- The input stream of bytes to decrypt.  
/// os --- The writer to output the decrypted bytes to.
///
/// # Errors
///
/// An error of "Bad Key." will be returned if `key` is not in the range `[0, 26)`.
///
/// # Notes
///
/// Values of `a-z` will not be changed by this function.
pub fn decrypt(key: u8, is: impl Iterator<Item = u8>, os: &mut Write) -> Result<()> {
    if key < ALPHA_LEN {
        //The key is a valid shift.
        //The temporary buffer.
        let mut buf = [0];
        let mut decrypt = |byte: u8| {
            //Place the byte in the temporary buffer.
            buf[0] = byte;
            
            //If the byte is ascii-alphabetic, it gets decrypted.
            if buf[0].is_ascii_uppercase() {
                //Shift the byte.
                buf[0] -= key;
                
                if buf[0] < b'A' {
                    //The byte was pushed out of the ascii-alphabetic range.
                    //Wrap the byte around to `Z`.
                    buf[0] += ALPHA_LEN;
                }
            }
            
            //Write the byte out.
            os.write_all(&buf)
        };
        
        for byte in is {
            //Decrypt the byte.
            decrypt(byte)?;
        }
        
        Ok(())
    } else {
        //The key is not a valid shift.
        Err(Error::new(ErrorKind::Other, "Bad Key."))
    }
}

#[cfg(test)]
mod tests {
    const PLAIN: &str = "The red fox, jumped, over the lazy dog! 0123456789";
    const CYPHER: &[u8] = b"DRO BON PYH, TEWZON, YFOB DRO VKJI NYQ! 0123456789";
    const KEY: u8 = 10;
    
    #[test]
    fn test_encrypt() {
        let mut cypher = Vec::with_capacity(PLAIN.len());
        
        ::caesar::encrypt(0, PLAIN.as_bytes().iter().map(Clone::clone), &mut cypher)
            .expect("Failed to encrypt plain-text.");
        assert_eq!(cypher, PLAIN.to_uppercase().as_bytes(), "Incorrect cypher-text.");
        
        cypher.clear();
        ::caesar::encrypt(KEY, PLAIN.as_bytes().iter().map(Clone::clone), &mut cypher)
            .expect("Failed to encrypt plain-text.");
        assert_eq!(cypher, CYPHER, "Incorrect cypher-text.");
    }
    
    #[test]
    fn test_decrypt() {
        let mut plain = Vec::with_capacity(PLAIN.len());
        
        ::caesar::decrypt(0, PLAIN.to_uppercase().as_bytes().iter().map(Clone::clone), &mut plain)
            .expect("Failed to decrypt cypher-text.");
        assert_eq!(plain, PLAIN.to_uppercase().as_bytes(), "Incorrect plain-text.");
        
        plain.clear();
        ::caesar::decrypt(KEY, CYPHER.iter().map(Clone::clone), &mut plain)
            .expect("Failed to decrypt cypher-text.");
        assert_eq!(plain, PLAIN.to_uppercase().as_bytes(), "Incorrect plain-text.");
    }
}
