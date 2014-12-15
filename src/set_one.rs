//imports
use std::collections::TreeMap;
use std::io::BufferedReader;
use std::io::File;
use std::num::Float;

/// all this set's challenges
fn main(){
	println!("set one ::");
	//challenge_one();
	//challenge_two();
	//challenge_three();
	//challenge_four();
	//challenge_five();
	challenge_six();
	//challenge_seven();
	//challenge_eight();
	println!("done set one");}

//challenges
/// solution for challenge one
fn challenge_one(){
	println!("challenge one ::");
	//params
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let desired = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	//get result
	let result = hexstr_tob64str( input);
	//print info
	//println!("input: {}", input);
	println!("result: {}", result);
	println!("result == desired: {}", result.as_slice() == desired);
	println!("done challenge one");}

/// solution for challenge two
fn challenge_two(){
	println!("challenge two ::");
	//params
	let input_a = "1c0111001f010100061a024b53535009181c";
	let input_b = "686974207468652062756c6c277320657965";
	let desired = "746865206b696420646f6e277420706c6179";
	//convert to bytes
	let a_bytes = hexstr_tobytes( input_a);
	let b_bytes = hexstr_tobytes( input_b);
	//apply xor
	let result_bytes = xor_bytes(
		a_bytes.as_slice(), b_bytes.as_slice());
	//convert to string
	let result = bytes_tohexstr( result_bytes.as_slice());
	//print info
	println!("result: {}", result);
	println!("result == desired: {}", result.as_slice() == desired);
	println!("done challenge two");}

/// solution for challenge three
fn challenge_three(){
	println!("challenge three ::");
	//params
	let input =
		"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
	let extractions = crack_xor_cipher( input);
	println!("extractions:");
	println!("cipher :: message :: rating");
	for ( &cipher, &( ref message, ref rating)) in extractions.iter() {
		println!("{:02x} :: {} :: {:.2}", cipher, message, *rating);}
	println!("done challenge three");}

/// solution for challenge four
fn challenge_four(){
	println!("challenge four ::");
	let lines = read_lines( "data/4.txt");
	let mut i : int = 0;
	for ref line in lines.iter() {
		let extractions = crack_xor_cipher( line.as_slice());
		if ! extractions.is_empty() {
			println!("extractions for line {}:", i);
			for ( &cipher, &( ref message, ref rating)) in extractions.iter() {
				println!("{:02x} :: {} :: {:.2}", cipher, message, *rating);}}
		i += 1;}
	println!("done challenge four");}

/// solution for challenge five
fn challenge_five(){
	println!("challenge five ::");
	//params
	let input =
		"Burning 'em, if you ain't quick and nimble\n".to_string() +
		"I go crazy when I hear a cymbal";
	let desired =
		"0b3637272a2b2e63622c2e69692a23693a2a3c".to_string() +
		"6324202d623d63343c2a26226324272765272a" +
		"282b2f20430a652e2c652a3124333a653e2b20" +
		"27630c692b20283165286326302e27282f";
	let key = "ICE";
	//convert to bytes
	let input_bytes = ascii_tobytes( input.as_slice());
	let key_bytes = ascii_tobytes( key);
	//apply repeating-key xor cipher
	let result_bytes = rkxor_cipher(
		input_bytes.as_slice(), key_bytes.as_slice());
	//convert to string
	let result = bytes_tohexstr( result_bytes.as_slice());
	//print info
	println!("result: {}", result);
	println!("result == desired: {}", result == desired);
	println!("done challenge five");}

/// solution for challenge six
fn challenge_six(){
	println!("challenge six ::");
	//hamming test
	let hamming_test0 = "this is a test";
	let hamming_test1 = "wokka wokka!!!";
	let hamming_desired = 37;
	let hamming_result = hamming_dist(
		ascii_tobytes( hamming_test0).as_slice(),
		ascii_tobytes( hamming_test1).as_slice());
	println!("hamming result: {}", hamming_result);
	println!("hamming result == desired: {}",
		hamming_result == hamming_desired);
	println!("test: {}",
		bytes_tob64codes( &[ 0xffu8, 0xff, 0xff]));
	println!("done challenge six");}

/// solution for challenge seven
fn challenge_seven(){
	println!("challenge seven ::");
	println!("done challenge seven");}

/// solution for challenge eight
fn challenge_eight(){
	println!("challenge eight ::");
	println!("done challenge eight");}


//conversion functions
/// convert a hex string into base64
fn hexstr_tob64str( input: &str) -> String {
	//vars
	let bytes = hexstr_tobytes( input);
	let b64_codes : Vec<u8> = bytes_tob64codes( bytes.as_slice());
	return b64codes_tob64str( b64_codes.as_slice());
	}

/// convert a hex string into a byte vector
fn hexstr_tobytes( input: &str) -> Vec<u8> {
	let mut i = 0;
	let mut bytes : Vec<u8> = Vec::new();
	//parse bytes
	while i < input.len() {
		//current byte we're parsing
		let mut byte = 0u8;
		//parse first half
		byte += hex_toi( input.char_at( i));
		byte <<= 4;
		i += 1;
		//break if byte is incomplete
		if i == input.len() {
			bytes.push( byte);
			break;}
		//parse second half
		byte += hex_toi( input.char_at( i));
		//add byte
		bytes.push( byte);
		i += 1;}
	return bytes;}

fn b64str_tobytes( input: &str) -> Vec<u8> {
	let mut codes : Vec<u8> = Vec::new();
	//convert to codes
	for character in input.chars() {
		codes.push( b64_toi( character));}
	//write codes into bytes
	//let mut byte : u8 = 0;
	//let mut bytes : Vec<u8> = Vec::new();
	let code_count = codes.len();
	for i in range( 0, code_count) {
		let code = codes[ i];}
	return Vec::new();}

fn bytes_tohexstr( bytes : &[u8]) -> String {
	let mut result = String::new();
	for &byte in bytes.iter() {
		let half1 = ( byte >> 4) & 0b1111;
		let half2 = byte & 0b1111;
		result.push( hexcode_tochar( half1));
		result.push( hexcode_tochar( half2));}
	return result;}

/// convert a byte string into b64 codes
fn bytes_tob64codes( bytes : &[u8]) -> Vec<u8> {
	let mut codes : Vec<u8> = Vec::new();
	//write bytes into b64 codes
	let mut leftover: u8 = 0;
	let byte_count = bytes.len();
	for i in range( 0, byte_count) {
		let byte = bytes[ i];
		match i % 3 {
			0 => {
				codes.push( byte >> 2);
				leftover = byte & 0b11;}
			1 => {
				codes.push( ( (leftover << 4) + (byte >> 4)));
				leftover = byte & 0b1111;}
			_ => {
				codes.push( ( (leftover << 2) + (byte >> 6)));
				codes.push( ( byte & 0b111111));}}}
	//write in the leftover and padding
	println!("leftover: {:x}", leftover);
	match byte_count % 3 {
		0 => codes.push_all( &[ leftover << 4, 64, 64]),
		1 => codes.push_all( &[ leftover << 2, 64]),
		_ => ()}
	return codes;}

fn b64codes_tob64str( codes : &[u8]) -> String {
	let mut result = String::new();
	//convert b64 codes into string
	for i in range( 0, codes.len()) {
		result.push( b64code_tochar( codes[ i]));}
	return result;}

/// convert a hex char its corresponding int
fn hex_toi( character : char) -> u8 {
	match character {
		'0'...'9' => (character as u8) - ('0' as u8) + 0,
		'a'...'f' => (character as u8) - ('a' as u8) + 10,
		_ => panic!("error: input contained non-hex chars")}}

/// convert a hex char its corresponding int
fn b64_toi( character : char) -> u8 {
	match character {
		'A'...'Z' => (character as u8) - ('A' as u8) + 0,
		'a'...'z' => (character as u8) - ('a' as u8) + 26,
		'0'...'9' => (character as u8) - ('0' as u8) + 53,
		'+' => 62,
		'/' => 63,
		'=' => 64,
		_ => panic!("error: input contained non-hex chars")}}

/// convert a base64 code into its corresponding char
fn b64code_tochar( code : u8) -> char {
	match code {
		00...25 => ( code - 00 + ('A' as u8)) as char,
		26...51 => ( code - 26 + ('a' as u8)) as char,
		52...61 => ( code - 52 + ('0' as u8)) as char,
		62 => '+',
		63 => '/',
		64 => '=',
		_ => panic!("error: code out of range")}}

/// convert a hec
fn hexcode_tochar( code : u8) -> char {
	match code {
		00...09 => ( code - 00 + ('0' as u8)) as char,
		10...15 => ( code - 10 + ('a' as u8)) as char,
		_ => panic!("error: code out of range")}}

fn ascii_tobytes( string : &str) -> Vec<u8> {
	let mut result = Vec::new();
	for character in string.chars() {
		result.push( character.to_ascii().as_byte());}
	return result;}

//utilities
/// xor two byte arrays
fn xor_bytes( a : &[u8], b : &[u8]) -> Vec<u8> {
	let size = std::cmp::min( a.len(), b.len());
	let mut result = Vec::new();
	for i in range( 0, size) {
		result.push( a[i] ^ b[i]);}
	return result;}

/// return a map of the bytes to their occurance counts
fn score_bytes( bytes : &[u8]) -> TreeMap<u8, u8> {
	let mut table : TreeMap<u8, u8> = TreeMap::new();
	for &byte in bytes.iter() {
		let new_count =
			match table.get( &byte) {
				Some( &count) => count + 1,
				None => 1};
		table.insert( byte, new_count);}
	return table;}

/// single byte xor cipher
fn xor_cipher( original : &[u8], cipher : u8) -> Vec<u8> {
	let mut result = Vec::new();
	for &byte in original.iter() {
		result.push( byte ^ cipher);}
	return result;}

/// repeating key xor cipher
fn rkxor_cipher( original : &[u8], cipher : &[u8]) -> Vec<u8> {
	let mut result = Vec::new();
	let ks = cipher.len();
	for i in range( 0, original.len()) {
		result.push( original[i] ^ cipher[ i % ks ]);}
	return result;}

/// try to break single byte xor cipher
fn crack_xor_cipher( input : &str) -> TreeMap< u8, (String, f32)> {
	//convert input to bytes
	let input_bytes = hexstr_tobytes( input);
	//create score table
	let input_scores = score_bytes( input_bytes.as_slice());
	//find top scores
	let mut top_scores : Vec<u8> = Vec::new();
	let scores_cap = 3;
	for &value in input_scores.values() {
		top_scores.push( value);}
	top_scores.sort();
	top_scores.reverse();
	let top_scores = top_scores.slice_to( scores_cap);
	//find bytes that match top scores
	let mut common_bytes : Vec<u8> = Vec::new();
	for ( &byte, score) in input_scores.iter() {
		if top_scores.contains( score) {
			common_bytes.push( byte);}}
	//result of above:
	//  most common bytes in input_bytes: { 120: 6, 55: 5, 54: 3 }
	let common_bytes = common_bytes.as_slice();
	//let common_chars = [' ','e','t','a','o','i','n','s','h','r','d','l','u'];
	let common_chars = [' ', 'e', 't', 'a', 'o', 'i'];
	let mut extractions : TreeMap< u8, (String, f32)> = TreeMap::new();
	let rating_cap : f32 = 0.8;
	//for every common byte and common char
	for &common_byte in common_bytes.iter() {
		for &common_char in common_chars.iter() {
			//interpret the cipher that would map the byte to the char
			let common_char_byte = common_char.to_ascii().as_byte();
			let cipher_byte = common_char_byte ^ common_byte;
			//check if we've already got this cipher
			if extractions.contains_key( &cipher_byte) {
				continue;}
			//create attempt
			let attempt_bytes = xor_cipher( input_bytes.as_slice(), cipher_byte);
			let attempt_slice = attempt_bytes.as_slice();
			if ! attempt_slice.is_ascii() {
				continue;}
			let attempt_ascii = attempt_slice.to_ascii();
			let attempt = attempt_ascii.as_str_ascii();
			//trim off any crud
			for &trimchar in ['\n',' ','\t'].iter() {
				attempt.trim_chars( trimchar);}
			//try to filter attempt
			let rating  = rate_attempt( attempt);
			if rating > rating_cap {
				//store the attempt
				extractions.insert(
					cipher_byte, ( String::from_str( attempt), rating));}}}
	//done
	return extractions;}

/// find the hamming distance of two byte arrays
fn hamming_dist( a : &[u8], b : &[u8]) -> u32 {
	if a.len() != b.len() {
		panic!("mismatching array lengths");}
	let mut result = 0;
	for i in range( 0, a.len()){
		let mut temp = a[i] ^ b[i];
		while temp != 0 {
			result += 1;
			temp &= temp - 1;}}
	return result;}

/// rate the likelihood a string is a good decoding attempt
fn rate_attempt( attempt : &str) -> f32 {
	//params
	let good_chars = "abcdefghijklmnopqrstuvwxyz ,.?!\'\"";
	let neut_chars = "()[]{}<>-=_+:;/@#$%^&*~`";
	let mut positive : f32 = 0.0;
	let mut negative : f32 = 0.0;
	let count = attempt.len();
	//count good and (probably) bad chars
	for attempt_char in attempt.chars() {
		//convert to lowercase for convenience
		let char_lower = attempt_char.to_lowercase();
		//if char is good, +1
		if good_chars.contains_char( char_lower) {
			positive += 1.0;}
		//if char is bad ( not good nor neutral ), -1
		else if ! neut_chars.contains_char( char_lower) {
			negative += 1.0;}}
	//add it all up in a nice way
	let numerator = positive.powi( 2) - negative.powi( 2);
	let denominator = count.to_f32().unwrap().powi( 2);
	return numerator / denominator;}

/// read every line in file, and write result
fn read_lines( filename : &str) -> Vec<String> {
	// open the file
	let path = Path::new( filename);
	let file = File::open( &path);
	let mut reader = BufferedReader::new( file);
	//read the file
	let mut result : Vec<String> = Vec::new();
	for line_result in reader.lines() {
		if ! line_result.is_ok() {
			panic!( "reading a line failed");}
		let line = line_result.unwrap();
		//strip the newline chars
		let line_stripped = line.as_slice().trim_chars('\n');
		result.push( String::from_str( line_stripped));}
	return result;}
