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
