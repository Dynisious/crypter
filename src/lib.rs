//! `crypter` is an API of common encryption algorithms.
//!
//! Author --- daniel.bechaz@gmail.com  
//! Last Modified --- 2018/01/09

#![feature(universal_impl_trait)]

#[cfg(test)]
extern crate hex;

pub mod caesar;
pub mod xor;
