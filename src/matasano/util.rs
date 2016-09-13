// imports
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn read_lines( file : File) -> Box<Iterator<Item=String>> {
	let file = BufReader::new( file);
	return Box::new( file.lines()
		.take_while( | x | x.is_ok())
		.map( | x | x.unwrap()));}
