// imports
use std::ascii::AsciiExt;
use std::collections::{ BTreeMap, BTreeSet};
use std::iter::{ FromIterator, repeat};

// local imports
use super::*;

// try to break single byte xor cipher
pub fn crack_xor_cipher( input : &Bytes) -> BTreeMap< u8, (String, i32)> {
	let common_cap = 5;
	let rating_cap : i32 = 0800;
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
		//try to filter attempt
		let rating  = rate_attempt( &attempt);
		if rating > rating_cap {
			//store the attempt
			extractions.insert(
				cipher, ( String::from_utf8( attempt).unwrap(), rating));}}
	// done
	return extractions;}

pub fn crack_rkxor( bytes: &Bytes) -> Bytes {
	// guess key sizes
	let key_sizes = guess_key_size( bytes);

	// choose best key guess for each key size
	let mut guesses : Vec<(Bytes, i32)> = Vec::new();
	for key_size in key_sizes {
		// partition into blocks
		let blocks = blockify( bytes, key_size);
		// choose best byte for each block
		let key = blocks.iter()
			.map( | block | guess_xor_cipher( block))
			.collect();
			// extract and rate candidate plain text
		let plain_text = bytes.xor_cipher( &key);
		let ranking = rate_attempt( &plain_text);
		guesses.push( (key, ranking));}

	// select best guess
	let key = guesses.iter()
		.max_by_key( | x | x.1)
		.unwrap().0.clone();
	return key;}

// find hamming distance of two byte iterators 
pub fn hamming_dist<'a, 'b, A, B>( a : A , b : B) -> u32
		where A : IntoIterator<Item=&'a u8>, B : IntoIterator<Item=&'b u8> {
	a.into_iter().zip( b.into_iter())
		.fold( 0, | acc, (a, b) | acc + ( a ^ b).count_ones())}

// partition byte array into transposed blocks
fn blockify( bytes: &[u8], size: usize) -> Vec<Bytes> {
	let mut blocks = vec![ Bytes::new(); size];
	for (&byte, i) in bytes.iter().zip( (0..size).cycle()) {
		blocks[i].push( byte);}
	return blocks;}

// guess most likely key sizes for cipher text based on hamming distance
fn guess_key_size( bytes: &[u8]) -> Vec<usize> {
	// create guesses
	let guesses : Vec<usize> = (2..40).collect();

	// rate guesses
	let mut ratings = vec![];
	for &size in &guesses {
		// select first four chunks
		let a : Bytes = bytes.iter()
			.take( size).cloned().collect();
		let b : Bytes = bytes.iter()
			.skip( size).take( size).cloned().collect();
		let c : Bytes = bytes.iter()
			.skip( 2 * size).take( size).cloned().collect();
		let d : Bytes = bytes.iter()
			.skip( 3 * size).take( size).cloned().collect();
		// rate guess based on average hamming distance between each chunk
		let rating = (
			hamming_dist( &a, &b) +
			hamming_dist( &a, &c) +
			hamming_dist( &a, &d) +
			hamming_dist( &b, &c) +
			hamming_dist( &b, &d) +
			hamming_dist( &c, &d)) / ( 6 * size as u32);
		ratings.push( (size, rating));}

	// select top 5 guesses
	ratings.sort_by_key( | x | x.1);
	let best = ratings.iter()
		.take( 5)
		.map( | x | x.0.clone())
		.collect();
	return best;}

// try to guess single byte xor cipher
pub fn guess_xor_cipher( input : &Bytes) -> u8 {
	let common_cap = 10;
	// create score table
	let input_scores = score_bytes( input);
	// find most common bytes
	let mut scores : Vec<(u8,u8)> =
		input_scores.iter()
		.map( | x | (x.0.clone(), x.1.clone())).collect();
	scores.sort_by( | a, b | b.1.cmp( &a.1));
	let common_bytes = scores.iter().map( | x | x.0).take( common_cap);

	// construct likely ciphers
	let mut ciphers = Vec::new();
	let common_chars : Vec<u8> = [' ', 'e', 't', 'a', 'o', 'i']
		.iter().map( | &c | c as u8).collect();
	let ciphers_iter = common_bytes
		.flat_map( | a | repeat( a).zip( common_chars.iter()))
		.map( | (a, b) | a ^ b);
	for cipher in ciphers_iter {
		if ! ciphers.contains( &cipher) {
			ciphers.push( cipher)}}

	let rankings = ciphers.iter().map( | cipher |{
		let attempt = input.xor_byte( cipher);
		return (cipher, rate_attempt( &attempt));});
	let guess = rankings
		.max_by_key( | x | x.1).unwrap()
		.0.clone();
	return guess;}

// return a map of the bytes to their occurance counts
fn score_bytes( bytes : &[u8]) -> BTreeMap<u8, u8> {
	let mut table : BTreeMap<u8, u8> = BTreeMap::new();
	for &byte in bytes {
		let count = table.get( &byte).unwrap_or( &0) + 1;
		table.insert( byte, count);}
	return table;}

// rate the likelihood a string is a good decoding attempt
fn rate_attempt( attempt : &Bytes) -> i32 {
	// check if attempt is possible
	if ! attempt.is_ascii() { return -1000;}
	let attempt = String::from_utf8( attempt.clone());
	if attempt.is_err() { return -1000;}
	let attempt = attempt.unwrap();

	// params
	let good_chars = BTreeSet::from_iter(
		"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.?!\'\"\n"
		.chars());
	let neut_chars = BTreeSet::from_iter(
		"()[]{}<>-=_+:;/@#$%^&*~`\r".chars());

	// count good and (probably) bad chars
	let mut good = 0.0f32;
	let mut bad = 0.0f32;
	let count = attempt.len();
	for attempt_char in attempt.chars() {
		// if char is good, +1
		if good_chars.contains( &attempt_char) {
			good += 1.0;}
		// if char is bad ( not good nor neutral ), -1
		else if ! neut_chars.contains( &attempt_char) {
			bad += 1.0;}}
	// add it all up in a nice way
	let numerator = good.powi( 2) - bad.powi( 2);
	let denominator = ( count as f32).powi( 2);
	return ( 1000.0 * numerator / denominator) as i32;}