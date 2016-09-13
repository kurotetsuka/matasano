// libraries
extern crate matasano;

// imports
use std::fs::File;

// local imports
use matasano::bytes::*;
use matasano::bytes::analysis::*;
use matasano::util::*;

fn main(){
	let file = File::open( "data/set1/4.txt").unwrap();
	/*let file = BufReader::new( file);
	let lines = file.lines().take_while( | x | x.is_ok())
		.map( | s | Bytes::from_hexstr( &s.unwrap()))
		.take_while( | x | x.is_some())
		.map( | x | x.unwrap());*/
	let lines = read_lines( file)
		.map( | s | Bytes::from_hexstr( &s))
		.take_while( | x | x.is_some())
		.map( | x | x.unwrap());
	println!("extractions: line :: cipher :: message :: rating");
	for (i, line) in (0..).zip( lines) {
		let extractions = crack_xor_cipher( &line);
		for ( &cipher, &( ref message, ref rating)) in extractions.iter() {
			println!(
				"{:03} :: {:02x} :: {:<40} :: {:.2}",
				i, cipher, format!( "{:?}", message), rating);}}
}
