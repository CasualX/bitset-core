use super::BitSet;

impl<T> BitSet for std::vec::Vec<T> where [T]: BitSet {
	impl_bitset!();
}
impl<T> BitSet for std::boxed::Box<[T]> where [T]: BitSet {
	impl_bitset!();
}
