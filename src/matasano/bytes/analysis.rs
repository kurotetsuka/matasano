// imports
use std::ascii::AsciiExt;
use std::collections::{ BTreeMap, BTreeSet};
use std::iter::{ FromIterator, repeat};

// local imports
use super::*;

// try to break single byte xor cipher
pub fn crack_xor_cipher( input : &Bytes) -> BTreeMap< u8, (String, f32)> {
	let common_cap = 5;
	let rating_cap : f32 = 0.8;
	// create score table
	let input_scores = score_bytes( input);
	// find most common bytes
	let mut entries : Vec<(u8,u8)> =
		input_scores.iter().map( | x | (x.0.clone(), x.1.clone())).collect();
	entries.sort_by( | a, b | b.1.cmp( &a.1));
	let common_bytes = entries.iter().map( | x | x.0).take( common_cap);
	let common_chars : Vec<u8> = [' ', 'e', 't', 'a', 'o', 'i']
		.iter().map( | &c | c as u8).collect();
	// construct likely ciphers
	let ciphers = common_bytes
		.flat_map( | a | repeat( a).zip( common_chars.iter()))
		.map( | (a, b) | a ^ b);

	// find likely plain-text
	let mut extractions = BTreeMap::new();
	for cipher in ciphers {
		// check if we've already got this cipher
		if extractions.contains_key( &cipher) { continue;}
		// create attempt
		let attempt = input.xor_byte( &cipher);
		if ! attempt.is_ascii() { continue;}
		let attempt = String::from_utf8( attempt);
		if attempt.is_err() { continue;}
		let attempt = attempt.unwrap();
		//try to filter attempt
		let rating  = rate_attempt( &attempt);
		if rating > rating_cap {
			//store the attempt
			extractions.insert(
				cipher, ( attempt, rating));}}
	// done
	return extractions;}

// find hamming distance of two byte iterators 
pub fn hamming_dist<'a, 'b, A, B>( a : A , b : B ) -> u32
		where A : IntoIterator<Item=&'a u8>, B : IntoIterator<Item=&'b u8> {
	a.into_iter().zip( b.into_iter())
		.fold( 0, | acc, (a, b) | acc + ( a ^ b).count_ones())}

/// return a map of the bytes to their occurance counts
fn score_bytes( bytes : &[u8]) -> BTreeMap<u8, u8> {
	let mut table : BTreeMap<u8, u8> = BTreeMap::new();
	for &byte in bytes {
		let count = table.get( &byte).unwrap_or( &0) + 1;
		table.insert( byte, count);}
	return table;}

// rate the likelihood a string is a good decoding attempt
fn rate_attempt( attempt : &str) -> f32 {
	// params
	let good_chars = BTreeSet::from_iter(
		"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.?!\'\""
		.chars());
	let neut_chars = BTreeSet::from_iter(
		"()[]{}<>-=_+:;/@#$%^&*~`".chars());
	let mut positive = 0.0f32;
	let mut negative = 0.0f32;
	let count = attempt.len();
	// count good and (probably) bad chars
	for attempt_char in attempt.chars() {
		// if char is good, +1
		if good_chars.contains( &attempt_char) {
			positive += 1.0;}
		// if char is bad ( not good nor neutral ), -1
		else if ! neut_chars.contains( &attempt_char) {
			negative += 1.0;}}
	// add it all up in a nice way
	let numerator = positive.powi( 2) - negative.powi( 2);
	let denominator = ( count as f32).powi( 2);
	return numerator / denominator;}
