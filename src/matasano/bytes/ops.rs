
// imports
use std::iter::Iterator;
use std::iter::repeat;
use super::*;

pub trait ByteOps {
	fn xor_bytes( &self, other: &Bytes) -> Self;
	fn xor_byte( &self, other: &u8) -> Self;
	fn xor_cipher( &self, other: &Bytes) -> Self;
}

impl ByteOps for Bytes {
	fn xor_bytes( &self, other: &Bytes) -> Self {
		xor( self.iter(), other.iter())}
	fn xor_byte( &self, key: &u8) -> Self{
		xor( self.iter(), repeat( key))}
	fn xor_cipher( &self, key: &Bytes) -> Self{
		xor( self.iter(), key.iter().cycle())}
}

pub fn xor<'a, A, B>( a : A , b : B ) -> Bytes
		where A : Iterator<Item=&'a u8>, B : Iterator<Item=&'a u8> {
	a.zip( b)
		.map( |(a, b)|{ a ^ b})
		.collect()}
fn find_min<'a, I>(vals: I) -> Option<&'a u8>
    where I: Iterator<Item=&'a u8>
{
    vals.min()
}