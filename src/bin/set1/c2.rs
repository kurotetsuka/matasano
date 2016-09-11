// libraries
extern crate matasano;

// imports
use matasano::bytes::*;

fn main(){
	let a = "1c0111001f010100061a024b53535009181c";
	let b = "686974207468652062756c6c277320657965";
	let a = Bytes::from_hexstr( a).unwrap();
	let b = Bytes::from_hexstr( b).unwrap();
	let c = a.xor_bytes( &b).to_hexstr();
	let desired = "746865206b696420646f6e277420706c6179";
	println!( "result: {:?}", c);
	println!( "result == desired ?: {}", c == desired);
}
