// compiler switches
#![allow(
	unused_imports,
	unused_variables,
)]

// libraries
extern crate matasano;

// imports
use std::collections::BTreeMap;

// local imports
use matasano::bytes::*;

fn main(){
	let common = [' ', 'e', 't', 'a', 'o', 'i'];
	let bytes = common.iter().map( | &c | c as u8);
	println!( "{:?}", common);
	for byte in bytes {
		println!( "{:02}", byte ^ 101u8);}
}
