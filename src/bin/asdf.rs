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
	let bytes : Bytes = (0..80).collect();
	let blocks = blockify( &bytes, 5);
	for block in blocks {
		println!( "block: {:?}", block);}
}

fn blockify( bytes: &[u8], size: usize) -> Vec<Bytes> {
	let mut blocks = vec![ Bytes::new(); size];
	for (&byte, i) in bytes.iter().zip( (0..size).cycle()) {
		blocks[i].push( byte);}
	println!( "blocks size: {}", blocks.len());
	println!( "blocks[0] size: {}", blocks[0].len());
	return blocks;}
