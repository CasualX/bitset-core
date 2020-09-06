use super::BitSet;

macro_rules! impl_bit_set_uint {
	($ty:ty, $bits_per_word:literal) => {
		impl BitSet for $ty {
			#[inline]
			fn bit_len(&self) -> usize {
				$bits_per_word
			}
			#[inline]
			fn bit_init(&mut self, value: bool) -> &mut Self {
				*self = <$ty>::wrapping_add(!(value as $ty), 1);
				self
			}
			#[inline]
			fn bit_test_usize(&self, bit: usize) -> bool {
				*self & (1 << bit as u32) != 0
			}
			#[inline]
			fn bit_set_usize(&mut self, bit: usize) -> &mut Self {
				*self |= 1 << bit as u32;
				self
			}
			#[inline]
			fn bit_reset_usize(&mut self, bit: usize) -> &mut Self {
				*self &= !(1 << bit as u32);
				self
			}
			#[inline]
			fn bit_flip_usize(&mut self, bit: usize) -> &mut Self {
				*self ^= 1 << bit as u32;
				self
			}
			#[inline]
			fn bit_cond_usize(&mut self, bit: usize, value: bool) -> &mut Self {
				let mask = 1 << bit as u32;
				*self = (*self & !mask) | (<$ty>::wrapping_add(!(value as $ty), 1) & mask);
				self
			}
			#[inline]
			fn bit_all(&self) -> bool {
				*self == !0
			}
			#[inline]
			fn bit_any(&self) -> bool {
				*self != 0
			}
			#[inline]
			fn bit_none(&self) -> bool {
				*self == 0
			}
			#[inline]
			fn bit_eq(&self, rhs: &Self) -> bool {
				*self == *rhs
			}
			#[inline]
			fn bit_disjoint(&self, rhs: &Self) -> bool {
				*self & *rhs == 0
			}
			#[inline]
			fn bit_subset(&self, rhs: &Self) -> bool {
				*self | *rhs == *rhs
			}
			#[inline]
			fn bit_superset(&self, rhs: &Self) -> bool {
				*self | *rhs == *self
			}
			#[inline]
			fn bit_or(&mut self, rhs: &Self) -> &mut Self {
				*self |= *rhs;
				self
			}
			#[inline]
			fn bit_and(&mut self, rhs: &Self) -> &mut Self {
				*self &= *rhs;
				self
			}
			#[inline]
			fn bit_andnot(&mut self, rhs: &Self) -> &mut Self {
				*self &= !*rhs;
				self
			}
			#[inline]
			fn bit_xor(&mut self, rhs: &Self) -> &mut Self {
				*self ^= *rhs;
				self
			}
			#[inline]
			fn bit_not(&mut self) -> &mut Self {
				*self = !*self;
				self
			}
			#[inline]
			fn bit_mask(&mut self, rhs: &Self, mask: &Self) -> &mut Self {
				*self = *self & !*mask | *rhs & *mask;
				self
			}
			#[inline]
			fn bit_count(&self) -> usize {
				self.count_ones() as usize
			}
		}
	};
}

impl_bit_set_uint!(u8, 8);
impl_bit_set_uint!(u16, 16);
impl_bit_set_uint!(u32, 32);
impl_bit_set_uint!(u64, 64);
impl_bit_set_uint!(u128, 128);

//----------------------------------------------------------------

#[test]
fn tests() {
	let mut bytes = 0u8;
	let mut words = 0u16;
	let mut dwords = 0u32;
	let mut qwords = 0u64;

	super::unary_tests(&mut bytes);
	super::unary_tests(&mut words);
	super::unary_tests(&mut dwords);
	super::unary_tests(&mut qwords);
}
