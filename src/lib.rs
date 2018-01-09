//! `crypter` is an API of common encryption algorithms.
//!
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/01/09

#![feature(universal_impl_trait)]

pub mod caesar;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
