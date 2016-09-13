// libraries
extern crate matasano;

// imports
use matasano::bytes::*;
use matasano::bytes::analysis::*;

fn main(){
	let line =
		"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
	let line = Bytes::from_hexstr( line).unwrap();
	let extractions = crack_xor_cipher( &line);
	println!("extractions: cipher :: message :: rating");
	for ( &cipher, &( ref message, ref rating)) in extractions.iter() {
		println!("{:02x} :: {} :: {:.2}", cipher, message, rating);}
}
