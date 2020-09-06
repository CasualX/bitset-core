
/// Types used as index into BitSets.
///
/// Can be extended for custom enumeration-like types.
///
/// # Examples
///
/// ```
/// use bitset_core::{BitIndex, BitSet};
///
/// // Let's define a custom C enum wrapper.
/// #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// pub struct MyEnum(pub u32);
///
/// impl MyEnum {
/// 	pub const VALUE: MyEnum = MyEnum(13);
/// }
///
/// // Make it usable as a BitSet index type.
/// impl BitIndex for MyEnum {
/// 	#[inline]
/// 	fn into_index(self) -> usize { self.0 as usize }
/// }
///
/// // Finally put it use.
/// let mut set = [0u8; 32];
/// set.bit_set(MyEnum::VALUE);
/// ```
pub trait BitIndex {
	fn into_index(self) -> usize;
}

macro_rules! impl_bit_index {
	($ty:ident) => {
		impl BitIndex for $ty {
			#[inline]
			fn into_index(self) -> usize {
				self as usize
			}
		}
	};
}

impl_bit_index!(usize);
impl_bit_index!(u32);
impl_bit_index!(u16);
impl_bit_index!(u8);
impl_bit_index!(isize);
impl_bit_index!(i32);
impl_bit_index!(i16);
impl_bit_index!(i8);

/// Derive macro implementing `BitIndex` for newtype wrappers.
#[macro_export]
macro_rules! BitIndex {
	(
		$(#[$struct_meta:meta])*
		$struct_vis:vis struct $struct_name:ident($(#[$field_meta:meta])* $field_vis:vis $field:ty);
	) => {
		impl $crate::BitIndex for $struct_name {
			#[inline]
			fn into_index(self) -> usize {
				self.0 as usize
			}
		}
	};
}
