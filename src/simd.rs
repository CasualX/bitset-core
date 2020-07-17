use super::BitSet;

// Implement SIMD version by relying on very optimizer friendly code
// Alignment is ignored but cam be taken care of by user code
macro_rules! impl_bit_set_simd {
	([[$elem_ty:ident; $elem_len:literal]], $bits_per_word:literal, [$($idx:tt)*]) => {
		impl BitSet for [[$elem_ty; $elem_len]] {
			#[inline]
			fn bit_len(&self) -> usize {
				self.len() * $bits_per_word
			}
			#[inline]
			fn bit_init(&mut self, value: bool) -> &mut Self {
				let value = [$elem_ty::wrapping_add(!(value as $elem_ty), 1); $elem_len];
				for i in 0..self.len() {
					self[i] = value;
				}
				self
			}
			#[inline]
			fn bit_test(&self, bit: usize) -> bool {
				let index = bit / $bits_per_word;
				let lane = (bit / ($bits_per_word / $elem_len)) % $elem_len;
				let mask = 1 << bit % ($bits_per_word / $elem_len);
				self[index][lane] & mask != 0
			}
			#[inline]
			fn bit_set(&mut self, bit: usize) -> &mut Self {
				let index = bit / $bits_per_word;
				let lane = (bit / ($bits_per_word / $elem_len)) % $elem_len;
				let mask = 1 << bit % ($bits_per_word / $elem_len);
				self[index][lane] |= mask;
				self
			}
			#[inline]
			fn bit_reset(&mut self, bit: usize) -> &mut Self {
				let index = bit / $bits_per_word;
				let lane = (bit / ($bits_per_word / $elem_len)) % $elem_len;
				let mask = 1 << bit % ($bits_per_word / $elem_len);
				self[index][lane] &= !mask;
				self
			}
			#[inline]
			fn bit_flip(&mut self, bit: usize) -> &mut Self {
				let index = bit / $bits_per_word;
				let lane = (bit / ($bits_per_word / $elem_len)) % $elem_len;
				let mask = 1 << bit % ($bits_per_word / $elem_len);
				self[index][lane] ^= mask;
				self
			}
			#[inline]
			fn bit_cond(&mut self, bit: usize, value: bool) -> &mut Self {
				let index = bit / $bits_per_word;
				let lane = (bit / ($bits_per_word / $elem_len)) % $elem_len;
				let mask = 1 << bit % ($bits_per_word / $elem_len);
				self[index][lane] = (self[index][lane] & !mask) | ($elem_ty::wrapping_add(!(value as $elem_ty), 1) & mask);
				self
			}
			#[inline]
			fn bit_all(&self) -> bool {
				for i in 0..self.len() {
					if self[i] != [!0; $elem_len] {
						return false;
					}
				}
				true
			}
			#[inline]
			fn bit_any(&self) -> bool {
				for i in 0..self.len() {
					if self[i] != [0; $elem_len] {
						return true;
					}
				}
				false
			}
			#[inline]
			fn bit_eq(&self, rhs: &Self) -> bool {
				assert_eq!(self.len(), rhs.len());
				for i in 0..self.len() {
					if self[i] != rhs[i] {
						return false;
					}
				}
				true
			}
			#[inline]
			fn bit_disjoint(&self, rhs: &Self) -> bool {
				assert_eq!(self.len(), rhs.len());
				for i in 0..self.len() {
					let tmp = [$(self[i][$idx] & rhs[i][$idx]),*];
					if tmp != [0; $elem_len] {
						return false;
					}
				}
				true
			}
			#[inline]
			fn bit_subset(&self, rhs: &Self) -> bool {
				assert_eq!(self.len(), rhs.len());
				for i in 0..self.len() {
					let tmp = [$(self[i][$idx] | rhs[i][$idx]),*];
					if tmp != rhs[i] {
						return false;
					}
				}
				true
			}
			#[inline]
			fn bit_or(&mut self, rhs: &Self) -> &mut Self {
				assert_eq!(self.len(), rhs.len());
				for i in 0..self.len() {
					$(self[i][$idx] |= rhs[i][$idx];)*
				}
				self
			}
			#[inline]
			fn bit_and(&mut self, rhs: &Self) -> &mut Self {
				assert_eq!(self.len(), rhs.len());
				for i in 0..self.len() {
					$(self[i][$idx] &= rhs[i][$idx];)*
				}
				self
			}
			#[inline]
			fn bit_andnot(&mut self, rhs: &Self) -> &mut Self {
				assert_eq!(self.len(), rhs.len());
				for i in 0..self.len() {
					$(self[i][$idx] &= !rhs[i][$idx];)*
				}
				self
			}
			#[inline]
			fn bit_xor(&mut self, rhs: &Self) -> &mut Self {
				assert_eq!(self.len(), rhs.len());
				for i in 0..self.len() {
					$(self[i][$idx] ^= rhs[i][$idx];)*
				}
				self
			}
			#[inline]
			fn bit_not(&mut self) -> &mut Self {
				for i in 0..self.len() {
					$(self[i][$idx] = !self[i][$idx];)*
				}
				self
			}
			#[inline]
			fn bit_mask(&mut self, rhs: &Self, mask: &Self) -> &mut Self {
				let len = self.len();
				assert_eq!(len, rhs.len());
				assert_eq!(len, mask.len());
				for i in 0..len {
					$(self[i][$idx] = self[i][$idx] & !mask[i][$idx] | rhs[i][$idx] & mask[i][$idx];)*
				}
				self
			}
			#[inline]
			fn bit_count(&self) -> usize {
				let mut result = [0; $elem_len];
				for i in 0..self.len() {
					$(result[$idx] += self[i][$idx].count_ones() as usize;)*
				}
				0 $(+result[$idx])*
			}
		}
	};
}

// simd128
impl_bit_set_simd!([[ u8; 16]], 128, [0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15]);
impl_bit_set_simd!([[u16;  8]], 128, [0 1 2 3 4 5 6 7]);
impl_bit_set_simd!([[u32;  4]], 128, [0 1 2 3]);
impl_bit_set_simd!([[u64;  2]], 128, [0 1]);

// simd256
impl_bit_set_simd!([[ u8; 32]], 256, [0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31]);
impl_bit_set_simd!([[u16; 16]], 256, [0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15]);
impl_bit_set_simd!([[u32;  8]], 256, [0 1 2 3 4 5 6 7]);
impl_bit_set_simd!([[u64;  4]], 256, [0 1 2 3]);

//----------------------------------------------------------------

#[test]
fn tests128() {
	let mut bytes = [[0u8; 16]; 32];
	let mut words = [[0u16; 8]; 16];
	let mut dwords = [[0u32; 4]; 8];
	let mut qwords = [[0u64; 2]; 4];

	super::unary_tests(&mut bytes[..]);
	super::unary_tests(&mut words[..]);
	super::unary_tests(&mut dwords[..]);
	super::unary_tests(&mut qwords[..]);
}

#[test]
fn tests256() {
	let mut bytes = [[0u8; 32]; 32];
	let mut words = [[0u16; 16]; 16];
	let mut dwords = [[0u32; 8]; 8];
	let mut qwords = [[0u64; 4]; 4];

	super::unary_tests(&mut bytes[..]);
	super::unary_tests(&mut words[..]);
	super::unary_tests(&mut dwords[..]);
	super::unary_tests(&mut qwords[..]);
}
