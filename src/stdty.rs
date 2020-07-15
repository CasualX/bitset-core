use std::{mem, slice};
use super::{BitSet, BitFmt};

impl<T> BitSet for std::vec::Vec<T> where [T]: BitSet {
	impl_bitset!();
}
impl<T> BitSet for std::boxed::Box<[T]> where [T]: BitSet {
	impl_bitset!();
}

//----------------------------------------------------------------
// Sparse bitset implementations

macro_rules! impl_sparse_bitset {
	($ty:ident) => {

/// Implements the BitSet API for sparse bitsets.
///
/// This is interesting as certain operations are not supported for sparse bitsets.
/// It makes no sense to 'set all bits' or 'flip all bits' as that would make it a very dense bitset.
/// These operations simply panic.
///
/// Bits which were once set and then reset will remain allocated.
///
/// The length of the bitset is only limited by the limitation of the hardware.
/// Any caller which attempts to naively enumerate bits will horribly break.
///
/// Formatting is not supported until specialization is available.
impl<T> BitSet for std::collections::$ty<usize, T> where [T]: BitSet, T: Default {
	#[inline]
	fn bit_len(&self) -> usize {
		usize::MAX
	}

	fn bit_init(&mut self, value: bool) -> &mut Self {
		// Not supported...
		if value {
			unimplemented!();
		}
		self.clear();
		self
	}
	fn bit_fmt(&self) -> &BitFmt<Self> {
		unimplemented!()
	}

	#[inline(never)]
	fn bit_test(&self, bit: usize) -> bool {
		let index = bit / mem::size_of::<T>();
		let bit = bit % mem::size_of::<T>();
		self.get(&index).map(|storage| slice::from_ref(storage).bit_test(bit)).unwrap_or(false)
	}
	#[inline(never)]
	fn bit_set(&mut self, bit: usize) -> &mut Self {
		let index = bit / mem::size_of::<T>();
		let bit = bit % mem::size_of::<T>();
		let storage = self.entry(index).or_default();
		slice::from_mut(storage).bit_set(bit);
		self
	}
	#[inline(never)]
	fn bit_reset(&mut self, bit: usize) -> &mut Self {
		let index = bit / mem::size_of::<T>();
		let bit = bit % mem::size_of::<T>();
		if let Some(storage) = self.get_mut(&index) {
			slice::from_mut(storage).bit_reset(bit);
		}
		self
	}
	#[inline(never)]
	fn bit_flip(&mut self, bit: usize) -> &mut Self {
		let index = bit / mem::size_of::<T>();
		let bit = bit % mem::size_of::<T>();
		let storage = self.entry(index).or_default();
		slice::from_mut(storage).bit_flip(bit);
		self
	}
	#[inline]
	fn bit_cond(&mut self, bit: usize, value: bool) -> &mut Self {
		if value { self.bit_set(bit) }
		else { self.bit_reset(bit) }
	}

	fn bit_all(&self) -> bool {
		false
	}
	fn bit_any(&self) -> bool {
		self.values().any(|storage| slice::from_ref(storage).bit_any())
	}

	#[inline(never)]
	fn bit_eq(&self, other: &Self) -> bool {
		let def = T::default();
		// Check if every element in self has corresponding bits set in other
		for (key, lhs) in self.iter() {
			let rhs = other.get(key).unwrap_or(&def);
			if !slice::from_ref(lhs).bit_eq(slice::from_ref(rhs)) {
				return false;
			}
		}
		// Check if every element in other has corresponding bits set in self
		for (key, rhs) in other.iter() {
			let lhs = self.get(key).unwrap_or(&def);
			if !slice::from_ref(lhs).bit_eq(slice::from_ref(rhs)) {
				return false;
			}
		}
		true
	}
	#[inline(never)]
	fn bit_disjoint(&self, other: &Self) -> bool {
		let def = T::default();
		// Check if every element in self has corresponding bits set in other
		for (key, lhs) in self.iter() {
			let rhs = other.get(key).unwrap_or(&def);
			if !slice::from_ref(lhs).bit_disjoint(slice::from_ref(rhs)) {
				return false;
			}
		}
		// Check if every element in other has corresponding bits set in self
		for (key, rhs) in other.iter() {
			let lhs = self.get(key).unwrap_or(&def);
			if !slice::from_ref(lhs).bit_disjoint(slice::from_ref(rhs)) {
				return false;
			}
		}
		true
	}
	#[inline(never)]
	fn bit_subset(&self, other: &Self) -> bool {
		let def = T::default();
		for (key, lhs) in self.iter() {
			let rhs = other.get(key).unwrap_or(&def);
			if !slice::from_ref(lhs).bit_subset(slice::from_ref(rhs)) {
				return false;
			}
		}
		true
	}

	#[inline(never)]
	fn bit_or(&mut self, other: &Self) -> &mut Self {
		for (&key, rhs) in other.iter() {
			slice::from_mut(self.entry(key).or_default()).bit_or(slice::from_ref(rhs));
		}
		self
	}
	#[inline(never)]
	fn bit_and(&mut self, other: &Self) -> &mut Self {
		for (key, rhs) in other.iter() {
			if let Some(lhs) = self.get_mut(key) {
				slice::from_mut(lhs).bit_and(slice::from_ref(rhs));
			}
		}
		self
	}
	#[inline(never)]
	fn bit_andnot(&mut self, other: &Self) -> &mut Self {
		for (key, rhs) in other.iter() {
			if let Some(lhs) = self.get_mut(key) {
				slice::from_mut(lhs).bit_andnot(slice::from_ref(rhs));
			}
		}
		self
	}
	#[inline(never)]
	fn bit_xor(&mut self, other: &Self) -> &mut Self {
		for (&key, rhs) in other.iter() {
			slice::from_mut(self.entry(key).or_default()).bit_or(slice::from_ref(rhs));
		}
		self
	}
	fn bit_not(&mut self) -> &mut Self {
		unimplemented!()
	}
	#[inline(never)]
	fn bit_mask(&mut self, other: &Self, mask: &Self) -> &mut Self {
		let def = T::default();
		for (key, mask) in mask.iter() {
			let lhs = self.entry(*key).or_default();
			let rhs = other.get(key).unwrap_or(&def);
			slice::from_mut(lhs).bit_mask(slice::from_ref(rhs), slice::from_ref(mask));
		}
		self
	}
	#[inline(never)]
	fn bit_count(&self) -> usize {
		self.values().map(|storage| slice::from_ref(storage).bit_count()).sum()
	}
}

};
}

impl_sparse_bitset!(HashMap);
impl_sparse_bitset!(BTreeMap);
