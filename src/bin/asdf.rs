// libraries
extern crate matasano;

// imports
use matasano::bytes::*;

fn main(){
	let a = "796f75";
	println!( "a: {}", a);
	let b = Bytes::from_hexstr( a).unwrap().to_hexstr();
	println!( "b: {}", b);
	let c = "c29ub3VzIG11c2hyb29t";
	println!( "c: {}", c);
	let d = Bytes::from_b64str( c).unwrap();
	println!( "d: {:?}", d);
	let e = d.to_b64str();
	println!( "e: {}", e);
}
