//! Consensus-encodable types
//!
//! This is basically a replacement of the `Encodable` trait which does
//! normalization for endianness, etc., to ensure that the encoding
//! matches for endianness, etc., to ensure that the encoding matches
//! the network consensus encoding.
//!
//! Essentially, anything that must go on the -disk- or -network- must
//! be encoded using the `Encodable` trait, since this data
//! must be the same for all systems. Any data going to the -user-, e.g.
//! over JSONRPC, should use the ordinary `Encodable` trait. (This
//! should also be the same across systems, of course, but has some
//! critical differences from the network format, e.g. scripts come
//! with an opcode decode, hashes are big-endian, numbers are typically
//! big-endian decimals, etc.)
//!

use crate::psgt;
use core::{convert::From, fmt, mem, u32};
use std::io::{self, Cursor, Read};

/// Encoding error
#[derive(Debug)]
pub enum Error {
	/// And I/O error
	Io(io::Error),
	/// PSGT-related error
	Psgt(psgt::Error),
	/// Network magic was not expected
	UnexpectedNetworkMagic {
		/// The expected network magic
		expected: u32,
		/// The unexpected network magic
		actual: u32,
	},
	/// Tried to allocate an oversized vector
	OversizedVectorAllocation {
		/// The capacity requested
		requested: usize,
		/// The maximum capacity
		max: usize,
	},
	/// Checksum was invalid
	InvalidChecksum {
		/// The expected checksum
		expected: [u8; 4],
		/// The invalid checksum
		actual: [u8; 4],
	},
	/// VarInt was encoded in a non-minimal way
	NonMinimalVarInt,
	/// Network magic was unknown
	UnknownNetworkMagic(u32),
	/// Parsing error
	ParseFailed(&'static str),
}

// TODO, consensus
/// Data which can be encoded in a consensus-consistent way
pub trait Encodable {
	/// Encode an object with a well-defined format.
	/// Returns the number of bytes written on success.
	///
	/// The only errors returned are errors propagated from the writer.
	fn encode<W: io::Write>(&self, writer: W) -> Result<usize, io::Error>;
}

/// Data which can be encoded in a consensus-consistent way
pub trait Decodable: Sized {
	/// Decode an object with a well-defined format
	fn decode<D: io::Read>(d: D) -> Result<Self, Error>;
}

/// Extensions of `Write` to encode data
pub trait WriteExt {
	/// Output a 64-bit uint
	fn emit_u64(&mut self, v: u64) -> Result<(), io::Error>;
	/// Output a 32-bit uint
	fn emit_u32(&mut self, v: u32) -> Result<(), io::Error>;
	/// Output a 16-bit uint
	fn emit_u16(&mut self, v: u16) -> Result<(), io::Error>;
	/// Output a 8-bit uint
	fn emit_u8(&mut self, v: u8) -> Result<(), io::Error>;

	/// Output a 64-bit int
	fn emit_i64(&mut self, v: i64) -> Result<(), io::Error>;
	/// Output a 32-bit int
	fn emit_i32(&mut self, v: i32) -> Result<(), io::Error>;
	/// Output a 16-bit int
	fn emit_i16(&mut self, v: i16) -> Result<(), io::Error>;
	/// Output a 8-bit int
	fn emit_i8(&mut self, v: i8) -> Result<(), io::Error>;

	/// Output a boolean
	fn emit_bool(&mut self, v: bool) -> Result<(), io::Error>;

	/// Output a byte slice
	fn emit_slice(&mut self, v: &[u8]) -> Result<(), io::Error>;
}

/// Extensions of `Read` to decode data
pub trait ReadExt {
	/// Read a 64-bit uint
	fn read_u64(&mut self) -> Result<u64, Error>;
	/// Read a 32-bit uint
	fn read_u32(&mut self) -> Result<u32, Error>;
	/// Read a 16-bit uint
	fn read_u16(&mut self) -> Result<u16, Error>;
	/// Read a 8-bit uint
	fn read_u8(&mut self) -> Result<u8, Error>;

	/// Read a 64-bit int
	fn read_i64(&mut self) -> Result<i64, Error>;
	/// Read a 32-bit int
	fn read_i32(&mut self) -> Result<i32, Error>;
	/// Read a 16-bit int
	fn read_i16(&mut self) -> Result<i16, Error>;
	/// Read a 8-bit int
	fn read_i8(&mut self) -> Result<i8, Error>;

	/// Read a boolean
	fn read_bool(&mut self) -> Result<bool, Error>;

	/// Read a byte slice
	fn read_slice(&mut self, slice: &mut [u8]) -> Result<(), Error>;
}

/// A variable-length unsigned integer
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct VarInt(pub u64);

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::Io(ref e) => write!(f, "I/O error: {}", e),
			Error::Psgt(ref e) => write!(f, "PSGT error: {}", e),
			Error::UnexpectedNetworkMagic {
				expected: ref e,
				actual: ref a,
			} => write!(f, "unexpected network magic: expected {}, actual {}", e, a),
			Error::OversizedVectorAllocation {
				requested: ref r,
				max: ref m,
			} => write!(
				f,
				"allocation of oversized vector: requested {}, maximum {}",
				r, m
			),
			Error::InvalidChecksum {
				expected: ref e,
				actual: ref a,
			} => write!(
				f,
				"invalid checksum: expected {}, actual {}",
				e.to_hex(),
				a.to_hex()
			),
			Error::NonMinimalVarInt => write!(f, "non-minimal varint"),
			Error::UnknownNetworkMagic(ref m) => write!(f, "unknown network magic: {}", m),
			Error::ParseFailed(ref e) => write!(f, "parse failed: {}", e),
		}
	}
}

#[cfg(feature = "std")]
impl ::std::error::Error for Error {
	fn cause(&self) -> Option<&dyn error::Error> {
		match *self {
			Error::Io(ref e) => Some(e),
			Error::Psgt(ref e) => Some(e),
			Error::UnexpectedNetworkMagic { .. }
			| Error::OversizedVectorAllocation { .. }
			| Error::InvalidChecksum { .. }
			| Error::NonMinimalVarInt
			| Error::UnknownNetworkMagic(..)
			| Error::ParseFailed(..)
			| Error::UnsupportedSegwitFlag(..) => None,
		}
	}
}

impl Decodable for VarInt {
	#[inline]
	fn decode<D: io::Read>(mut d: D) -> Result<Self, Error> {
		let n = ReadExt::read_u8(&mut d)?;
		match n {
			0xFF => {
				let x = ReadExt::read_u64(&mut d)?;
				if x < 0x100000000 {
					Err(self::Error::NonMinimalVarInt)
				} else {
					Ok(VarInt(x))
				}
			}
			0xFE => {
				let x = ReadExt::read_u32(&mut d)?;
				if x < 0x10000 {
					Err(self::Error::NonMinimalVarInt)
				} else {
					Ok(VarInt(x as u64))
				}
			}
			0xFD => {
				let x = ReadExt::read_u16(&mut d)?;
				if x < 0xFD {
					Err(self::Error::NonMinimalVarInt)
				} else {
					Ok(VarInt(x as u64))
				}
			}
			n => Ok(VarInt(n as u64)),
		}
	}
}

// Booleans
impl Encodable for bool {
	#[inline]
	fn encode<S: WriteExt>(&self, mut s: S) -> Result<usize, io::Error> {
		s.emit_bool(*self)?;
		Ok(1)
	}
}

impl Decodable for bool {
	#[inline]
	fn decode<D: io::Read>(mut d: D) -> Result<bool, Error> {
		ReadExt::read_bool(&mut d)
	}
}

// Strings
impl Encodable for String {
	#[inline]
	fn encode<S: io::Write>(&self, mut s: S) -> Result<usize, io::Error> {
		let b = self.as_bytes();
		let vi_len = VarInt(b.len() as u64).encode(&mut s)?;
		s.emit_slice(&b)?;
		Ok(vi_len + b.len())
	}
}

impl Decodable for String {
	#[inline]
	fn decode<D: io::Read>(d: D) -> Result<String, Error> {
		String::from_utf8(Decodable::decode(d)?)
			.map_err(|_| self::Error::ParseFailed("String was not valid UTF8"))
	}
}

#[doc(hidden)]
impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::Io(error)
	}
}

#[doc(hidden)]
impl From<psgt::Error> for Error {
	fn from(e: psgt::Error) -> Error {
		Error::Psgt(e)
	}
}

/// Encode an object into a vector
pub fn serialize<T: Encodable + ?Sized>(data: &T) -> Vec<u8> {
	let mut encoder = Vec::new();
	let len = data.encode(&mut encoder).unwrap();
	debug_assert_eq!(len, encoder.len());
	encoder
}

/// Deserialize an object from a vector, will error if said deserialization
/// doesn't consume the entire vector.
pub fn deserialize<T: Decodable>(data: &[u8]) -> Result<T, Error> {
	let (rv, consumed) = deserialize_partial(data)?;

	// Fail if data are not consumed entirely.
	if consumed == data.len() {
		Ok(rv)
	} else {
		Err(Error::ParseFailed(
			"data not consumed entirely when explicitly deserializing",
		))
	}
}

/// Deserialize an object from a vector, but will not report an error if said deserialization
/// doesn't consume the entire vector.
pub fn deserialize_partial<T: Decodable>(data: &[u8]) -> Result<(T, usize), Error> {
	let mut decoder = Cursor::new(data);
	let rv = Decodable::decode(&mut decoder)?;
	let consumed = decoder.position() as usize;

	Ok((rv, consumed))
}
