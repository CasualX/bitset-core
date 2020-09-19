/*!
BitSet
======

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/bitset-core.svg)](https://crates.io/crates/bitset-core)
[![docs.rs](https://docs.rs/bitset-core/badge.svg)](https://docs.rs/bitset-core)

Straightforward, no-std compatible, simd optimized, BitSet API.

Examples
--------

This crate provides its functionality through the `BitSet` trait.

```rust
use bitset_core::BitSet;
```

The containers for the bitset provided by this crate are unsigned integers, slices of unsigned integers and simd-like types, and `Vec<_>`, `Box<[_]>` if the `std` feature is enabled (enabled by default).

```rust
use bitset_core::BitSet;

let mut bits = [0u32; 4];
assert_eq!(bits.bit_len(), 4 * 32);

bits.bit_init(true); // Set all bits to true
assert!(bits.bit_all()); // All bits are set

bits.bit_reset(13); // Reset the 13th bit
assert!(bits.bit_any()); // At least some bits are set

bits.bit_flip(42); // Flip the 42nd bit twice (no change)
bits.bit_flip(42);

bits.bit_cond(1, false); // Set the bit to runtime value

assert_eq!(bits.bit_test(42), true);
assert_eq!(bits.bit_test(13), false);
assert_eq!(bits.bit_test(1), false);

assert_eq!(bits.bit_count(), 4 * 32 - 2);
```

Simd optimization is provided by using underlying primitives such as `[u32; 4]` which match the hardware's 128-bit simd registers. The compiler is heavily encouraged to vectorize these primitives.

```rust
use bitset_core::BitSet;

let mut a = [[0x21212121u32; 4]; 16];
let b = [[0x55555555u32; 4]; 16];

a.bit_or(&b);
a.bit_and(&b);
a.bit_xor(&b);
a.bit_not();

assert_eq!(a, [[0xffffffffu32; 4]; 16]);
```

For non fixed-size containers using the `std` feature `BitSet` is also implemented for `Vec<T>` and `Box<[T]>` (where `[T]`: `BitSet`).

Future work includes making everything const fn to enable all of this at compiletime, blocked on support for traits in const fn.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
*/

#![allow(incomplete_features)]
#![feature(const_trait_impl, const_fn, const_mut_refs, const_panic)]
#![no_std]

#[cfg(any(test, feature = "std"))]
#[macro_use]
extern crate std;

/// The BitSet API.
pub trait BitSet {
	/// Returns total number of bits.
	fn bit_len(&self) -> usize;

	/// Initializes all bits.
	fn bit_init(&mut self, value: bool) -> &mut Self;
	/// Format the bits.
	#[inline]
	fn bit_fmt(&self) -> &BitFmt<Self> {
		unsafe { &*(self as *const _ as *const _) }
	}

	/// Returns if the given bit is set.
	fn bit_test(&self, bit: usize) -> bool;
	/// Sets the given bit.
	fn bit_set(&mut self, bit: usize) -> &mut Self;
	/// Resets the given bit.
	fn bit_reset(&mut self, bit: usize) -> &mut Self;
	/// Flips the given bit.
	fn bit_flip(&mut self, bit: usize) -> &mut Self;
	/// Conditionally sets or resets the given bit.
	fn bit_cond(&mut self, bit: usize, value: bool) -> &mut Self;

	/// Returns if all bits are set.
	fn bit_all(&self) -> bool;
	/// Returns if any bits are set.
	fn bit_any(&self) -> bool;
	/// Returns if none of the bits are set.
	#[inline]
	fn bit_none(&self) -> bool {
		!self.bit_any()
	}

	/// Returns if the two bitsets are equal.
	fn bit_eq(&self, rhs: &Self) -> bool;
	/// Returns if the two bitsets have no bits in common.
	fn bit_disjoint(&self, rhs: &Self) -> bool;
	/// Returns if self is a subset of rhs.
	fn bit_subset(&self, rhs: &Self) -> bool;
	/// Returns if self is a superset of rhs.
	#[inline]
	fn bit_superset(&self, rhs: &Self) -> bool {
		rhs.bit_subset(self)
	}

	/// Bitwise OR.
	fn bit_or(&mut self, rhs: &Self) -> &mut Self;
	/// Bitwise AND.
	fn bit_and(&mut self, rhs: &Self) -> &mut Self;
	/// Bitwise AND after NOT of rhs.
	fn bit_andnot(&mut self, rhs: &Self) -> &mut Self;
	/// Bitwise XOR.
	fn bit_xor(&mut self, rhs: &Self) -> &mut Self;
	/// Bitwise NOT.
	fn bit_not(&mut self) -> &mut Self;
	/// Bitwise combine with MASK.
	fn bit_mask(&mut self, rhs: &Self, mask: &Self) -> &mut Self;

	/// Counts the number of set bits.
	fn bit_count(&self) -> usize;
}

/// Shorthand for setting bits on the bitset container.
///
/// Returns the value of the initial argument after setting the bits.
///
/// ```
/// use bitset_core::{bitset, BitSet};
/// let bits = bitset!([0u8; 4]; 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31);
/// assert_eq!(bits.bit_count(), 11);
/// ```
#[macro_export]
macro_rules! bitset {
	($init:expr; $($bit:expr),* $(,)?) => {{
		use $crate::BitSet;
		#[allow(unused_mut)]
		match $init {
			mut this => {
				$(this.bit_set($bit as usize);)*
				this
			},
		}
	}};
}

/// Shorthand for combining bitsets with bit_or.
///
/// Returns the value of the initial argument after combining the bits.
///
/// ```
/// use bitset_core::{bitor, BitSet};
/// let bits = bitor!([0u8; 4]; [0x01; 4], [0x10; 4]);
/// assert_eq!(bits, [0x11; 4]);
/// ```
#[macro_export]
macro_rules! bitor {
	($init:expr; $($bits:expr),* $(,)?) => {{
		use $crate::BitSet;
		#[allow(unused_mut)]
		match $init {
			mut this => {
				$(this.bit_or(&$bits);)*
				this
			},
		}
	}};
}

/// Implements the `BitSet` trait members for your type through `DerefMut`.
///
/// Unfortunately due to the trait orphan rules it is not possible to automatically provide an implementation for all types implementing DerefMut.
#[macro_export]
macro_rules! impl_bitset {
	() => {
		#[inline]
		fn bit_len(&self) -> usize {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_len(<Self as ops::Deref>::deref(self))
		}

		#[inline]
		fn bit_init(&mut self, value: bool) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_init(<Self as ops::DerefMut>::deref_mut(self), value);
			self
		}

		#[inline]
		fn bit_test(&self, bit: usize) -> bool {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_test(<Self as ops::Deref>::deref(self), bit)
		}
		#[inline]
		fn bit_set(&mut self, bit: usize) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_set(<Self as ops::DerefMut>::deref_mut(self), bit);
			self
		}
		#[inline]
		fn bit_reset(&mut self, bit: usize) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_reset(<Self as ops::DerefMut>::deref_mut(self), bit);
			self
		}
		#[inline]
		fn bit_flip(&mut self, bit: usize) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_flip(<Self as ops::DerefMut>::deref_mut(self), bit);
			self
		}
		#[inline]
		fn bit_cond(&mut self, bit: usize, value: bool) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_cond(<Self as ops::DerefMut>::deref_mut(self), bit, value);
			self
		}

		#[inline]
		fn bit_all(&self) -> bool {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_all(<Self as ops::Deref>::deref(self))
		}
		#[inline]
		fn bit_any(&self) -> bool {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_any(<Self as ops::Deref>::deref(self))
		}
		#[inline]
		fn bit_none(&self) -> bool {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_none(<Self as ops::Deref>::deref(self))
		}

		#[inline]
		fn bit_eq(&self, rhs: &Self) -> bool {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_eq(<Self as ops::Deref>::deref(self), <Self as ops::Deref>::deref(rhs))
		}
		#[inline]
		fn bit_disjoint(&self, rhs: &Self) -> bool {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_disjoint(<Self as ops::Deref>::deref(self), <Self as ops::Deref>::deref(rhs))
		}
		#[inline]
		fn bit_subset(&self, rhs: &Self) -> bool {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_subset(<Self as ops::Deref>::deref(self), <Self as ops::Deref>::deref(rhs))
		}
		#[inline]
		fn bit_superset(&self, rhs: &Self) -> bool {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_superset(<Self as ops::Deref>::deref(self), <Self as ops::Deref>::deref(rhs))
		}

		#[inline]
		fn bit_or(&mut self, rhs: &Self) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_or(<Self as ops::DerefMut>::deref_mut(self), <Self as ops::Deref>::deref(rhs));
			self
		}
		#[inline]
		fn bit_and(&mut self, rhs: &Self) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_and(<Self as ops::DerefMut>::deref_mut(self), <Self as ops::Deref>::deref(rhs));
			self
		}
		#[inline]
		fn bit_andnot(&mut self, rhs: &Self) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_andnot(<Self as ops::DerefMut>::deref_mut(self), <Self as ops::Deref>::deref(rhs));
			self
		}
		#[inline]
		fn bit_xor(&mut self, rhs: &Self) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_xor(<Self as ops::DerefMut>::deref_mut(self), <Self as ops::Deref>::deref(rhs));
			self
		}
		#[inline]
		fn bit_not(&mut self) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_not(<Self as ops::DerefMut>::deref_mut(self));
			self
		}
		#[inline]
		fn bit_mask(&mut self, rhs: &Self, mask: &Self) -> &mut Self {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_mask(<Self as ops::DerefMut>::deref_mut(self), <Self as ops::Deref>::deref(rhs), <Self as ops::Deref>::deref(mask));
			self
		}

		#[inline]
		fn bit_count(&self) -> usize {
			use ::core::ops;
			<<Self as ops::Deref>::Target as $crate::BitSet>::bit_count(<Self as ops::Deref>::deref(self))
		}
	};
}

mod uint;
mod slice;
mod simd;

#[cfg(feature = "std")]
mod stdty;

mod fmt;
pub use self::fmt::BitFmt;

//----------------------------------------------------------------

#[cfg(test)]
fn unary_tests<T: ?Sized + BitSet>(bits: &mut T) {
	// reset all bits
	bits.bit_init(false);
	assert_eq!(bits.bit_any(), false);
	assert_eq!(bits.bit_all(), false);
	// set even bits
	for i in 0..bits.bit_len() {
		bits.bit_set(i & !1);
	}
	assert_eq!(bits.bit_any(), true);
	assert_eq!(bits.bit_all(), false);
	for i in 0..bits.bit_len() {
		assert_eq!(bits.bit_test(i), i & 1 == 0);
	}
	// invert all bits and flip back
	bits.bit_not();
	for i in 0..bits.bit_len() {
		bits.bit_flip(i);
		assert_eq!(bits.bit_test(i), i & 1 == 0);
	}

	// set all bits
	bits.bit_init(true);
	assert_eq!(bits.bit_any(), true);
	assert_eq!(bits.bit_all(), true);
	// clear even bits
	for i in 0..bits.bit_len() {
		bits.bit_reset(i & !1);
	}
	assert_eq!(bits.bit_any(), true);
	assert_eq!(bits.bit_all(), false);
	for i in 0..bits.bit_len() {
		assert_eq!(bits.bit_test(i), i & 1 != 0);
	}
	// invert all bits and flip back
	bits.bit_not();
	for i in 0..bits.bit_len() {
		bits.bit_flip(i);
		assert_eq!(bits.bit_test(i), i & 1 != 0);
	}

	assert!(!bits.bit_disjoint(bits));
	assert!(bits.bit_subset(bits));
	assert!(bits.bit_superset(bits));
}
