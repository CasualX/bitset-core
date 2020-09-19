use super::BitSet;

macro_rules! impl_bit_set_slice {
	([$elem_ty:ty], $bits_per_word:literal) => {
		impl const BitSet for [$elem_ty] {
			#[inline]
			fn bit_len(&self) -> usize {
				self.len() * $bits_per_word
			}

			#[inline]
			fn bit_init(&mut self, value: bool) -> &mut Self {
				let value = <$elem_ty>::wrapping_add(!(value as $elem_ty), 1);
				let mut i = 0;
				while i < self.len() {
					self[i] = value;
					i += 1;
				}
				self
			}

			#[inline]
			fn bit_test(&self, bit: usize) -> bool {
				self[bit / $bits_per_word] & (1 << bit % $bits_per_word) != 0
			}
			#[inline]
			fn bit_set(&mut self, bit: usize) -> &mut Self {
				self[bit / $bits_per_word] |= 1 << bit % $bits_per_word;
				self
			}
			#[inline]
			fn bit_reset(&mut self, bit: usize) -> &mut Self {
				self[bit / $bits_per_word] &= !(1 << bit % $bits_per_word);
				self
			}
			#[inline]
			fn bit_flip(&mut self, bit: usize) -> &mut Self {
				self[bit / $bits_per_word] ^= 1 << bit % $bits_per_word;
				self
			}
			#[inline]
			fn bit_cond(&mut self, bit: usize, value: bool) -> &mut Self {
				let index = bit / $bits_per_word;
				let mask = 1 << bit % $bits_per_word;
				self[index] = (self[index] & !mask) | (<$elem_ty>::wrapping_add(!(value as $elem_ty), 1) & mask);
				self
			}

			#[inline]
			fn bit_all(&self) -> bool {
				let mut i = 0;
				while i < self.len() {
					if self[i] != !0 {
						return false;
					}
					i += 1;
				}
				true
			}
			#[inline]
			fn bit_any(&self) -> bool {
				let mut i = 0;
				while i < self.len() {
					if self[i] != 0 {
						return true;
					}
					i += 1;
				}
				false
			}

			#[inline]
			fn bit_eq(&self, rhs: &Self) -> bool {
				assert!(self.len() == rhs.len());
				let mut i = 0;
				while i < self.len() {
					if self[i] != rhs[i] {
						return false;
					}
					i += 1;
				}
				true
			}
			#[inline]
			fn bit_disjoint(&self, rhs: &Self) -> bool {
				assert!(self.len() == rhs.len());
				let mut i = 0;
				while i < self.len() {
					if self[i] & rhs[i] != 0 {
						return false;
					}
					i += 1;
				}
				true
			}
			#[inline]
			fn bit_subset(&self, rhs: &Self) -> bool {
				assert!(self.len() == rhs.len());
				let mut i = 0;
				while i < self.len() {
					if self[i] | rhs[i] != rhs[i] {
						return false;
					}
					i += 1;
				}
				true
			}

			#[inline]
			fn bit_or(&mut self, rhs: &Self) -> &mut Self {
				assert!(self.len() == rhs.len());
				let mut i = 0;
				while i < self.len() {
					self[i] |= rhs[i];
					i += 1;
				}
				self
			}
			#[inline]
			fn bit_and(&mut self, rhs: &Self) -> &mut Self {
				assert!(self.len() == rhs.len());
				let mut i = 0;
				while i < self.len() {
					self[i] &= rhs[i];
					i += 1;
				}
				self
			}
			#[inline]
			fn bit_andnot(&mut self, rhs: &Self) -> &mut Self {
				assert!(self.len() == rhs.len());
				let mut i = 0;
				while i < self.len() {
					self[i] &= !rhs[i];
					i += 1;
				}
				self
			}
			#[inline]
			fn bit_xor(&mut self, rhs: &Self) -> &mut Self {
				assert!(self.len() == rhs.len());
				let mut i = 0;
				while i < self.len() {
					self[i] ^= rhs[i];
					i += 1;
				}
				self
			}
			#[inline]
			fn bit_not(&mut self) -> &mut Self {
				let mut i = 0;
				while i < self.len() {
					self[i] = !self[i];
					i += 1;
				}
				self
			}
			#[inline]
			fn bit_mask(&mut self, rhs: &Self, mask: &Self) -> &mut Self {
				let len = self.len();
				assert!(len == rhs.len());
				assert!(len == mask.len());
				let mut i = 0;
				while i < self.len() {
					self[i] = self[i] & !mask[i] | rhs[i] & mask[i];
					i += 1;
				}
				self
			}

			#[inline]
			fn bit_count(&self) -> usize {
				let mut result = 0;
				let mut i = 0;
				while i < self.len() {
					result += self[i].count_ones() as usize;
					i += 1;
				}
				result
			}
		}
	};
}

impl_bit_set_slice!([u8], 8);
impl_bit_set_slice!([u16], 16);
impl_bit_set_slice!([u32], 32);
impl_bit_set_slice!([u64], 64);
impl_bit_set_slice!([u128], 128);

//----------------------------------------------------------------

#[test]
fn tests() {
	let mut bytes = [0u8; 32];
	let mut words = [0u16; 16];
	let mut dwords = [0u32; 8];
	let mut qwords = [0u64; 4];

	super::unary_tests(&mut bytes[..]);
	super::unary_tests(&mut words[..]);
	super::unary_tests(&mut dwords[..]);
	super::unary_tests(&mut qwords[..]);
}

// Tests whether `bit X` is the same bit regardless of underlying primitive used.
// NOTE: This only works on little endian, nobody cares about big endian anyway.
#[test]
fn test_transmute() {
	for i in 0..32 {
		let uint = bitset!([0u32; 1]; i);
		let ubyte = bitset!([0u8; 4]; i);
		assert_eq!(uint[0], u32::from_ne_bytes(ubyte));
	}
}
