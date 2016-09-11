// libraries
extern crate matasano;

// imports
use matasano::bytes::*;

fn main(){
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let desired = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

	let bytes = Bytes::from_hexstr( input);
	if bytes.is_none() {
		println!( "bad input :(");
		return;}
	let bytes = bytes.unwrap();

	let result = bytes.to_b64str();
	println!( "result: {}", result);
	println!( "result == desired ?: {}", result == desired);

	let bytes2 = Bytes::from_b64str( desired);
	if bytes2.is_none() {
		println!( "bad input :(");
		return;}
	let bytes2 = bytes2.unwrap();

	let result2 = bytes2.to_hexstr();
	println!( "result2: {}", result2);
	println!( "result2 == input ?: {}", result2 == input);
}
