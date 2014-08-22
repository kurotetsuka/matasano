fn main(){
	//params
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	//let desired = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

	//vars
	let mut i = 0;
	let mut bytes : Vec<u8> = Vec::new();
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

	//write bytes into string
	let mut leftover: u8 = 0;
	for i in range( 0, bytes.len()) {
		let byte = bytes.get( i);
		match i % 3 {
			0 => {
				result.push_char( int_tob64( byte >> 2));
				leftover = byte & 0b11;}
			1 => {
				result.push_char( int_tob64( (leftover << 4) + (byte >> 4)));
				leftover = byte & 0b1111;}
			_ => {
				result.push_char( int_tob64( (leftover << 2) + (byte >> 6)));
				result.push_char( int_tob64( byte & 0b111111));}}}

	println!("Result: {}", result);
}

fn hex_toi( character : char) -> u8 {
	match character {
		'0'..'9' => (character as u8) - ('0' as u8) + 0,
		'a'..'f' => (character as u8) - ('a' as u8) + 10,
		_ => fail!("error: input contained non-hex chars")}}

fn int_tob64( i : u8) -> char {
	match i {
		00..25 => ( i + ('A' as u8)) as char,
		26..51 => ( i + ('a' as u8)) as char,
		52..61 => ( i + ('0' as u8)) as char,
		62 => '+',
		63 => '/',
		_ => fail!("error: byte out of range")}}