fn main(){
	//params
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let desired = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

	//vars
	let mut i = 0;
	let mut bytes : Vec<u8> = Vec::new();
	let mut b64_codes : Vec<u8> = Vec::new();
	let mut result = String::new();

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

	//write bytes into b64 codes
	let mut leftover: u8 = 0;
	for i in range( 0, bytes.len()) {
		let byte = bytes.get( i);
		match i % 3 {
			0 => {
				b64_codes.push( ( byte >> 2));
				leftover = byte & 0b11;}
			1 => {
				b64_codes.push( ( (leftover << 4) + (byte >> 4)));
				leftover = byte & 0b1111;}
			_ => {
				b64_codes.push( ( (leftover << 2) + (byte >> 6)));
				b64_codes.push( ( byte & 0b111111));}}}

	//convert b64 codes into string
	for i in range( 0, b64_codes.len()) {
		result.push_char( int_tob64( b64_codes.get( i)));}

	println!("input: {}", input);
	//println!("bytes: {}", bytes);
	//println!("b64_codes: {}", b64_codes);
	println!("result: {}", result);
	println!("result == desired: {}", result.as_slice() == desired);
}

fn hex_toi( character : char) -> u8 {
	match character {
		'0'..'9' => (character as u8) - ('0' as u8) + 0,
		'a'..'f' => (character as u8) - ('a' as u8) + 10,
		_ => fail!("error: input contained non-hex chars")}}

fn int_tob64( i : &u8) -> char {
	match *i {
		00..25 => ( i - 00 + ('A' as u8)) as char,
		26..51 => ( i - 26 + ('a' as u8)) as char,
		52..61 => ( i - 52 + ('0' as u8)) as char,
		62 => '+',
		63 => '/',
		_ => fail!("error: byte out of range")}}