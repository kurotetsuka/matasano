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
		xor( self, other)}
	fn xor_byte( &self, key: &u8) -> Self{
		xor( self, repeat( key))}
	fn xor_cipher( &self, key: &Bytes) -> Self{
		xor( self, key.iter().cycle())}
}

pub fn xor<'a, A, B>( a : A , b : B ) -> Bytes
		where A : IntoIterator<Item=&'a u8>, B : IntoIterator<Item=&'a u8> {
	a.into_iter().zip( b.into_iter())
		.map( | (a, b) | a ^ b)
		.collect()}
