use core::fmt;
use super::BitSet;

/// BitSet formatter.
#[repr(transparent)]
pub struct BitFmt<T: ?Sized>(T);

fn bitstring<T: ?Sized + BitSet>(this: &T, f: &mut fmt::Formatter) -> fmt::Result {
	const ALPHABET: [u8; 2] = [b'0', b'1'];
	let mut buf = [0u8; 9];
	let mut first = true;
	buf[0] = b'_';
	let mut i = 0;
	while i < this.bit_len() {
		buf[1] = ALPHABET[this.bit_test(i + 0) as usize];
		buf[2] = ALPHABET[this.bit_test(i + 1) as usize];
		buf[3] = ALPHABET[this.bit_test(i + 2) as usize];
		buf[4] = ALPHABET[this.bit_test(i + 3) as usize];
		buf[5] = ALPHABET[this.bit_test(i + 4) as usize];
		buf[6] = ALPHABET[this.bit_test(i + 5) as usize];
		buf[7] = ALPHABET[this.bit_test(i + 6) as usize];
		buf[8] = ALPHABET[this.bit_test(i + 7) as usize];
		let s = unsafe { &*((&buf[first as usize..]) as *const _ as *const str) };
		f.write_str(s)?;
		i += 8;
		first = false;
	}
	Ok(())
}

const UPPERHEX_ALPHABET: [u8; 16] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F'];
const LOWERHEX_ALPHABET: [u8; 16] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f'];

fn hexstring<T: ?Sized + BitSet>(this: &T, f: &mut fmt::Formatter, alphabet: &[u8; 16]) -> fmt::Result {
	let mut buf = [0u8; 2];
	let mut i = 0;
	while i < this.bit_len() {
		let byte =
			(this.bit_test(i + 0) as u8) << 7 |
			(this.bit_test(i + 1) as u8) << 6 |
			(this.bit_test(i + 2) as u8) << 5 |
			(this.bit_test(i + 3) as u8) << 4 |
			(this.bit_test(i + 4) as u8) << 3 |
			(this.bit_test(i + 5) as u8) << 2 |
			(this.bit_test(i + 6) as u8) << 1 |
			(this.bit_test(i + 7) as u8) << 0;
		buf[0] = alphabet[(byte >> 4) as usize];
		buf[1] = alphabet[(byte & 0xf) as usize];
		let s = unsafe { &*((&buf[..]) as *const _ as *const str) };
		f.write_str(s)?;
		i += 8;
	}
	Ok(())
}

impl<T: ?Sized + BitSet> fmt::Display for BitFmt<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		bitstring(&self.0, f)
	}
}
impl<T: ?Sized + BitSet> fmt::Debug for BitFmt<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("\"")?;
		hexstring(&self.0, f, &UPPERHEX_ALPHABET)?;
		f.write_str("\"")?;
		Ok(())
	}
}

impl<T: ?Sized + BitSet> fmt::UpperHex for BitFmt<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		hexstring(&self.0, f, &UPPERHEX_ALPHABET)
	}
}
impl<T: ?Sized + BitSet> fmt::LowerHex for BitFmt<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		hexstring(&self.0, f, &LOWERHEX_ALPHABET)
	}
}
