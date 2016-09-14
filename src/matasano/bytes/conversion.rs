// imports
use std::ascii::AsciiExt;

// local imports
use super::*;

pub trait FromAsciiStr {
	fn from_ascii( string: &str) -> Option<Self>
		where Self: Sized;
}
pub trait FromHexStr {
	fn from_hexstr( string: &str) -> Option<Self>
		where Self: Sized;
}
pub trait FromB64Str {
	fn from_b64str( string: &str) -> Option<Self>
		where Self: Sized;
}
pub trait ToAsciiStr {
	fn to_ascii( &self) -> Option<String>;
}
pub trait ToHexStr {
	fn to_hexstr( &self) -> String;
}
pub trait ToB64Str {
	fn to_b64str( &self) -> String;
}

impl FromAsciiStr for Bytes {
	fn from_ascii( string: &str) -> Option<Bytes> {
		if ! string.is_ascii() {
			None}
		else {
			Some( string.as_bytes().to_vec())}}
}
impl FromHexStr for Bytes {
	fn from_hexstr( string: &str) -> Option<Bytes> {
		return hexstr_to_bytes( string);}
}
impl FromB64Str for Bytes {
	fn from_b64str( string: &str) -> Option<Bytes> {
		return b64str_to_bytes( string);}
}
impl ToAsciiStr for Bytes {
	fn to_ascii( &self) -> Option<String> {
		if ! self.is_ascii() { None}
		else { String::from_utf8( self.clone()).ok()}}
}
impl ToHexStr for Bytes {
	fn to_hexstr( &self) -> String {
		return bytes_to_hexstr( self);}
}
impl ToB64Str for Bytes {
	fn to_b64str( &self) -> String {
		return bytes_to_b64str( self);}
}


pub fn hexstr_to_bytes( string: &str) -> Option<Vec<u8>> {
	// odd size check
	if string.len() % 2 != 0 {
		return None;}

	let mut chars = string.chars();
	let mut result = Vec::new();
	// get a pair of chars
	while let (Some( a), Some( b)) = (chars.next(), chars.next()) {
		// convert chars to ints
		if let (Some( a), Some( b)) = (a.to_digit( 16), b.to_digit( 16)) {
			// combine and push
			result.push( ( ( a << 4) + b) as u8);}
		// catch errors
		else { return None;}}
	// catch errors
	return Some( result);}

pub fn b64str_to_bytes( input: &str) -> Option<Vec<u8>> {
	// convert to codes
	let mut codes = Vec::<u8>::new();
	for character in input.chars() {
		if let Some( code) = char_to_b64code( character) {
			codes.push( code);}
		else { return None;}}
	// convert to bytes
	return b64codes_to_bytes( &codes);}

pub fn bytes_to_b64str( bytes: &[u8]) -> String {
	// convert to codes
	let codes = bytes_to_b64codes( bytes);
	// convert to string
	return b64codes_to_str( &codes);}

pub fn bytes_to_hexstr( bytes: &[u8]) -> String {
	let mut result = String::new();
	for byte in bytes {
		let byte = format!( "{:02x}", byte);
		result.push_str( &byte);}
	return result;}

fn bytes_to_b64codes( bytes : &[u8]) -> Vec<u8> {
	let mut codes : Vec<u8> = Vec::new();
	let mut leftover : u8 = 0;
	let byte_count = bytes.len();
	for i in 0..byte_count {
		let byte = bytes[ i];
		match i % 3 {
			0 => {
				codes.push( byte >> 2);
				leftover = byte & 0b11;}
			1 => {
				codes.push( ( ( leftover << 4) + ( byte >> 4)));
				leftover = byte & 0b1111;}
			_ => {
				codes.push( ( ( leftover << 2) + ( byte >> 6)));
				codes.push( ( byte & 0b111111));}}}
	// write in the leftover and padding
	match byte_count % 3 {
		1 => codes.extend( &[ leftover << 4, 64, 64]),
		2 => codes.extend( &[ leftover << 2, 64]),
		_ => ()}
	return codes;}

fn b64codes_to_str( codes : &[u8]) -> String {
	let mut result = String::new();
	//convert b64 codes into string
	for &x in codes {
		result.push( b64code_to_char( x));}
	return result;}

fn b64codes_to_bytes( codes : &[u8]) -> Option<Vec<u8>> {
	if codes.len() % 4 != 0 {
		return None;}
	let mut result = Vec::new();
	// convert codes into quads
	let mut codes = codes.iter();
	while let (Some( &a), Some( &b), Some( &c), Some( &d)) = 
			(codes.next(), codes.next(), codes.next(), codes.next()) {
		// error check
		if a == 64 || b == 64 { return None;}
		// push first byte
		result.push( ( a << 2) + ( b >> 4));
		if c == 64 { continue;}
		// push second byte
		result.push( ( ( b & 0b1111) << 4) + ( c >> 2));
		if d == 64 { continue;}
		// push third byte
		result.push( ( ( c & 0b11) << 6) + d);}
	// return
	return Some( result);}

/// convert a b64 char its corresponding int
fn char_to_b64code( character : char) -> Option<u8> {
	match character {
		'A'...'Z' => Some( (character as u8) - ('A' as u8) + 0),
		'a'...'z' => Some( (character as u8) - ('a' as u8) + 26),
		'0'...'9' => Some( (character as u8) - ('0' as u8) + 52),
		'+' => Some( 62),
		'/' => Some( 63),
		'=' => Some( 64),
		_ => None}}

/// convert a base64 code into its corresponding char
fn b64code_to_char( code : u8) -> char {
	match code {
		00...25 => ( code - 00 + ('A' as u8)) as char,
		26...51 => ( code - 26 + ('a' as u8)) as char,
		52...61 => ( code - 52 + ('0' as u8)) as char,
		62 => '+',
		63 => '/',
		64 => '=',
		_ => panic!()}}
