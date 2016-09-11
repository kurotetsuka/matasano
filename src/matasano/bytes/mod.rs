
// modules
mod analysis;
mod conv;
mod ops;

// re-exports
pub use self::ops::*;


pub type Bytes = Vec<u8>;

pub trait FromHexStr {
	fn from_hexstr( string: &str) -> Option<Bytes>;
}
pub trait FromB64Str {
	fn from_b64str( string: &str) -> Option<Bytes>;
}
pub trait ToHexStr {
	fn to_hexstr( &self) -> String;
}
pub trait ToB64Str {
	fn to_b64str( &self) -> String;
}


impl FromHexStr for Bytes {
	fn from_hexstr( string: &str) -> Option<Bytes> {
		return conv::hexstr_to_bytes( string);}
}
impl FromB64Str for Bytes {
	fn from_b64str( string: &str) -> Option<Bytes> {
		return conv::b64str_to_bytes( string);}
}
impl ToHexStr for Bytes {
	fn to_hexstr( &self) -> String {
		return conv::bytes_to_hexstr( self);}
}
impl ToB64Str for Bytes {
	fn to_b64str( &self) -> String {
		return conv::bytes_to_b64str( self);}
}
