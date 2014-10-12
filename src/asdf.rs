fn main(){
	let original : [u8, ..7] = [ 0u8, 1, 2, 3, 4, 5, 6];
	let original_copy = copy( original);}

fn copy( slice : &[u8]) -> [u8] {
	let result : Vec<u8> = Vec::new();
	for i in range( 0, slice.len()) {
		result.push( slice[i]);}
	return result.as_slice();}
