
use std::io::{Read, Write, Result, Error, ErrorKind};

pub const DEFAULT_BUF_LEN: usize = 10;

pub fn encrypt(key: u8, is: impl Read, os: &mut Write) -> Result<()> {
    if key < 26 {
        let mut buf = [0; 1];
        
        for byte in is.bytes() {
            buf[0] = byte?;
            
            if buf[0].is_ascii_alphabetic() {
                buf[0] = buf[0].to_ascii_uppercase() + key;
                
                if buf[0] > b'Z' {
                    buf[0] -= 26;
                }
            }
            
            os.write_all(&mut buf)?;
        }
        
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Bad Key."))
    }
}

pub fn decrypt(key: u8, is: impl Read, os: &mut Write) -> Result<()> {
    if key < 26 {
        let mut buf = [0; 1];
        
        for byte in is.bytes() {
            buf[0] = byte?;
            
            if buf[0].is_ascii_alphabetic() {
                buf[0] = buf[0].to_ascii_uppercase() - key;
                
                if buf[0] < b'A' {
                    buf[0] += 26;
                }
            }
            
            os.write_all(&mut buf)?;
        }
        
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Bad Key."))
    }
}

#[cfg(test)]
mod tests {
    const PLAIN: &str = "The red fox, jumped, over the lazy dog! 0123456789";
    const CYPHER: &str = "DRO BON PYH, TEWZON, YFOB DRO VKJI NYQ! 0123456789";
    
    #[test]
    fn test_encrypt() {
        let mut cypher = String::with_capacity(PLAIN.len());
        
        ::caesar::encrypt(0, PLAIN.as_bytes(), unsafe { cypher.as_mut_vec() })
            .expect("Failed to encrypt plain-text.");
        assert_eq!(cypher, PLAIN.to_uppercase(), "Incorrect cypher-text.");
        
        cypher.clear();
        ::caesar::encrypt(10, PLAIN.as_bytes(), unsafe { cypher.as_mut_vec() })
            .expect("Failed to encrypt plain-text.");
        assert_eq!(cypher, CYPHER, "Incorrect cypher-text.");
    }
    
    #[test]
    fn test_decrypt() {
        let mut plain = String::with_capacity(PLAIN.len());
        
        ::caesar::decrypt(0, PLAIN.as_bytes(), unsafe { plain.as_mut_vec() })
            .expect("Failed to decrypt cypher-text.");
        assert_eq!(plain, PLAIN.to_uppercase(), "Incorrect plain-text.");
        
        plain.clear();
        ::caesar::decrypt(10, CYPHER.as_bytes(), unsafe { plain.as_mut_vec() })
            .expect("Failed to decrypt cypher-text.");
        assert_eq!(plain, PLAIN.to_uppercase(), "Incorrect plain-text.");
    }
}
