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
