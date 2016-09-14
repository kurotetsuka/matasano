// libraries
extern crate matasano;

// local imports
use matasano::bytes::*;

fn main(){
	let input = format!(
		"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
	let input = Bytes::from_ascii( &input).unwrap();
	let key = Bytes::from_ascii( "ICE").unwrap();
	let cipher_text = input.xor_cipher( &key);
	let cipher_text = cipher_text.to_hexstr();
	let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
	println!( "cipher_text: {}", cipher_text);
	println!( "cipher_text == expect ?: {}", cipher_text == expected);
}
