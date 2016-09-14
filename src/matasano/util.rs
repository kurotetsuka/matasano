// imports
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

// local imports
use bytes::*;

pub fn read_lines( file : File) -> Box<Iterator<Item=String>> {
	let file = BufReader::new( file);
	return Box::new( file.lines()
		.take_while( | x | x.is_ok())
		.map( | x | x.unwrap()));}

pub fn read_b64_lines( file : File) -> Option<Bytes> {
	let file = BufReader::new( file);
	let mut text = String::new();
	for line in file.lines()
			.take_while( | x | x.is_ok())
			.map( | x | x.unwrap()) {
		text.push_str( &line);}
	return Bytes::from_b64str( &text);}
