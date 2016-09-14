// libraries
extern crate matasano;

// imports
use std::fs::File;

// local imports
use matasano::bytes::*;
use matasano::bytes::analysis::*;
use matasano::util::*;

fn main(){
	let a = Bytes::from_ascii( "this is a test").unwrap();
	let b = Bytes::from_ascii( "wokka wokka!!!").unwrap();
	println!( "hamming( a, b): {:?}", hamming_dist( &a, &b));

	let file = File::open( "data/set1/6.txt").unwrap();
	let cipher_text = read_b64_lines( file).unwrap();
	let (key, plain_text) = crack_rkxor( &cipher_text);
	println!( "key: {}", key.to_hexstr());
	let plain_text = plain_text.to_ascii().unwrap();
	println!( "plain_text: {}", plain_text);
}
