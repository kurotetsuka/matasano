fn main(){
	println!("set one ::");
	//challenge_one();
	challenge_two();
	//challenge_three();
	//challenge_four();
	//challenge_five();
	//challenge_six();
	//challenge_seven();
	//challenge_eight();
}

//challenges
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
	println!("result == desired: {}", result.as_slice() == desired);}

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
	/*let result_bytes = xor_bytes( a_bytes, b_bytes);
	//convert to string
	let result = bytes_tohexstr( result_bytes);
	//print info
	println!("result: {}", result);
	println!("result == desired: {}", result.as_slice() == desired);*/}
fn challenge_three(){
	println!("challenge three ::");}
fn challenge_four(){
	println!("challenge four ::");}
fn challenge_five(){
	println!("challenge five ::");}
fn challenge_six(){
	println!("challenge six ::");}
fn challenge_seven(){
	println!("challenge seven ::");}
fn challenge_eight(){
	println!("challenge eight ::");}

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
		//break if byte is \nincomplete
		if i == input.len() {
			bytes.push( byte);
			break;}
		//parse second half
		byte += hex_toi( input.char_at( i));
		//add byte
		bytes.push( byte);
		i += 1;}
	return bytes;}

/// convert a byte string into b64 codes
fn bytes_tob64codes( bytes : &[u8]) -> Vec<u8> {
	let mut codes : Vec<u8> = Vec::new();
	//write bytes into b64 codes
	let mut leftover: u8 = 0;
	for i in range( 0, bytes.len()) {
		let byte = bytes[ i];
		match i % 3 {
			0 => {
				codes.push( ( byte >> 2));
				leftover = byte & 0b11;}
			1 => {
				codes.push( ( (leftover << 4) + (byte >> 4)));
				leftover = byte & 0b1111;}
			_ => {
				codes.push( ( (leftover << 2) + (byte >> 6)));
				codes.push( ( byte & 0b111111));}}}
	return codes;}

fn b64codes_tob64str( codes : &[u8]) -> String {
	let mut result = String::new();
	//convert b64 codes into string
	for i in range( 0, codes.len()) {
		result.push_char( b64code_tochar( codes[ i]));}
	return result;}

/// convert a hex char its corresponding int
fn hex_toi( character : char) -> u8 {
	match character {
		'0'..'9' => (character as u8) - ('0' as u8) + 0,
		'a'..'f' => (character as u8) - ('a' as u8) + 10,
		_ => fail!("error: input contained non-hex chars")}}

/// convert a base64 code into its corresponding char
fn b64code_tochar( i : u8) -> char {
	match i {
		00..25 => ( i - 00 + ('A' as u8)) as char,
		26..51 => ( i - 26 + ('a' as u8)) as char,
		52..61 => ( i - 52 + ('0' as u8)) as char,
		62 => '+',
		63 => '/',
		_ => fail!("error: byte out of range")}}