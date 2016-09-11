#![crate_name="matasano"]
#![crate_type="lib"]

// features and plugins
//#![feature(
//	slice_patterns,
//	advanced_slice_patterns)]

// library imports
//extern crate asdf;

// reexports
pub use prelude::*;

// modules
pub mod bytes;
pub mod crypt;
pub mod prelude;
pub mod util;

pub fn test( x: u8){
	println!( "x: {}", x);}
