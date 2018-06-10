// MIT License

// Copyright (c) 2018 Arthur Maciejewicz

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! A dense bit set implemented over Vec<usize>
//!
//! # Examples
//! 
//! use bitsets::DenseBitSet
//! 
//! let mut bs = DenseBitSet::with_capacity(1024);
//! 
//! bs.set(5);
//! bs.set(6);
//! bs.set(15);
//! 
//! if (bs.test(5)) {
//!   println!("Hey it works!");
//! }
//! 
//! if (bs.test(13)) {
//!     println!("Hey it doesn't work");
//! }
//! 


use std::mem;

const BITS_PER_BYTE: usize = 8;
const BYTES_PER_WORD: usize = mem::size_of::<usize>();
const BITS_PER_WORD: usize = BYTES_PER_WORD * BITS_PER_BYTE;

#[inline]
fn get_word_offset(pos: usize) -> usize {
    pos / BITS_PER_WORD
}

#[inline]
fn get_bit_offset(pos: usize) -> usize {
    pos % BITS_PER_WORD
}

#[inline]
fn get_bitmask(pos: usize) -> usize {
    1 << get_bit_offset(pos)
}

/// A dense bit set implemented over Vec<usize> 
#[derive(Clone, Eq, PartialEq)]
pub struct DenseBitSet {
    num_bits: usize,
    bits: Vec<usize>,
}


impl DenseBitSet {
    /// Creates a `DenseBitSet` that can contain _at least_ num_bits bits
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bitsets::DenseBitSet;
    /// 
    /// let mut bs = DenseBitSet::with_capacity(128);
    /// 
    /// bs.set(1);
    /// bs.set(2);
    /// bs.set(3);
    /// 
    /// assert!(bs.test(1));
    /// assert!(bs.test(2));
    /// assert!(bs.test(3));
    /// ```
    pub fn with_capacity(num_bits: usize) -> DenseBitSet {
        let full_words = num_bits / BITS_PER_WORD;
        let remaining_bits = num_bits % BITS_PER_WORD;
        let words_to_allocate;
        if remaining_bits > 0 {
            words_to_allocate = full_words + 1;
        } else {
            words_to_allocate = full_words;
        }

        DenseBitSet {
            bits: vec![0; words_to_allocate],
            num_bits: words_to_allocate * BITS_PER_WORD,
        }
    }

    /// Sets the ith bit
    /// returns true if bit was not set previously
    pub fn set(&mut self, i: usize) -> bool {
        let idx = get_word_offset(i);
        let prior = self.bits[idx];
        let bitmask = get_bitmask(i);

        self.bits[idx] |= bitmask;
        (prior & bitmask) == 0
    }

    /// Tests whether the ith bit is set
    /// Returns true if is set, else false
    pub fn test(&self, i: usize) -> bool {
        (self.bits[get_word_offset(i)] & get_bitmask(i)) != 0
    }

    /// flips the value of the ith bit
    pub fn flip(&mut self, i: usize) {
        self.bits[get_word_offset(i)] ^= get_bitmask(i)
    }

    /// returns the number of elements in the underlying Vec<usize>
    pub fn words(&self) -> usize {
        self.bits.len()
    }

    /// returns the number of bits this set can accommodate
    pub fn len(&self) -> usize {
        self.num_bits
    }

    /// prints the bitset to STDOUT
    pub fn print(&self) {
        for i in 0..self.len() {
            print!("{}", if self.test(i) { 1 } else { 0 });
        }
        println!("");
    }

    /// In-place bitwise-not
    pub fn inplace_not(&mut self) {
        for i in 0..self.bits.len() {
            self.bits[i] = !self.bits[i];
        }
    }

    /**
     * 
     * Combining functions
     * 
     * The following methods take another bitset,
     * and merge the two together
     * 
     */

    /// In-place bitwise-and with `other`
    pub fn inplace_and(&mut self, other: &DenseBitSet) {
        assert!(self.words() == other.words());

        for i in 0..self.bits.len() {
            self.bits[i] &= other.bits[i];
        }
    }

    /// In-place bitwise-or with `other`
    pub fn inplace_or(&mut self, other: &DenseBitSet) {
        assert!(self.words() == other.words());

        for i in 0..self.bits.len() {
            self.bits[i] |= other.bits[i];
        }
    }

    /// In-place bitwise-xor with `other`
    pub fn inplace_xor(&mut self, other: &DenseBitSet) {
        assert!(self.words() == other.words());

        for i in 0..self.bits.len() {
            self.bits[i] ^= other.bits[i];
        }
    }

    pub fn and(&self, other: &DenseBitSet) -> DenseBitSet {
        assert!(self.words() == other.words());

        let mut output = self.clone();
        output.inplace_and(other);
        output
    }

    pub fn or(&self, other: &DenseBitSet) -> DenseBitSet {
        assert!(self.words() == other.words());
        
        let mut output = self.clone();
        output.inplace_or(other);
        output
    }

    pub fn xor(&self, other: &DenseBitSet) -> DenseBitSet {
        assert!(self.words() == other.words());
        let mut output = self.clone();
        output.inplace_xor(other);
        output
    }
}

// DenseBitSet TESTS
mod tests {
    
    use super::*;

    #[test]
    fn can_create() {
        let bs = DenseBitSet::with_capacity(128);

        assert!(bs.len() == 128);
    }

    #[test]
    fn can_set_and_test_bits() {
        let mut bs = DenseBitSet::with_capacity(128);
        assert_eq!(bs.test(0), false);
        assert_eq!(bs.test(10), false);
        assert_eq!(bs.test(30), false);
        bs.set(0);
        bs.set(10);
        bs.set(30);
        assert_eq!(bs.test(0), true);
        assert_eq!(bs.test(10), true);
        assert_eq!(bs.test(30), true);
    }

    #[test]
    fn can_report_num_words() {
        // initialize with non-aligned value
        let bs = DenseBitSet::with_capacity(100);
        assert_eq!(128, bs.len());
        assert_eq!(2, bs.words());
    }

    #[test]
    fn can_clear_bits() {
        let mut bs = DenseBitSet::with_capacity(64);

        bs.set(45);
        assert_eq!(bs.test(45), true);

        bs.flip(45);
        assert_eq!(bs.test(45), false);
    }
}