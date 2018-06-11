# bitsets

Various heap-allocated bitset implementations in Rust.

At the moment we provide a `DenseBitSet` datastructure, and plan to provide compressed and memory-mapped
bitsets in the near future.

## Usage

```rust
use bitsets::DenseBitSet;

let A = DenseBitSet::from_bits(0b1001100000100010);
let B = DenseBitSet::from_bits(0b1001100000100010);
let C = A.or(&B);
```

