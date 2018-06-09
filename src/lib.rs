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

pub struct BitSet {
    bits: Vec<usize>,
    num_bits: usize,
}

impl BitSet {
    pub fn with_capacity(num_bits: usize) -> BitSet {
        let full_words = num_bits / BITS_PER_WORD;
        let remaining_bits = num_bits % BITS_PER_WORD;
        let words_to_allocate;
        if remaining_bits > 0 {
            words_to_allocate = full_words + 1;
        } else {
            words_to_allocate = full_words;
        }

        BitSet {
            bits: vec![0; words_to_allocate],
            num_bits: words_to_allocate * BITS_PER_WORD,
        }
    }

    pub fn set(&mut self, bit: usize) -> bool {
        let idx = get_word_offset(bit);
        let prior = self.bits[idx];
        let bitmask = get_bitmask(bit);

        self.bits[idx] |= bitmask;
        (prior & bitmask) == 0
    }

    pub fn test(&self, bit: usize) -> bool {
        (self.bits[get_word_offset(bit)] & get_bitmask(bit)) != 0
    }

    pub fn toggle(&mut self, bit: usize) {
        self.bits[get_word_offset(bit)] ^= get_bitmask(bit)
    }

    pub fn words(&self) -> usize {
        self.bits.len()
    }

    pub fn len(&self) -> usize {
        self.num_bits
    }

    pub fn print(&self) {
        for i in 0..self.len() {
            print!("{}", if self.test(i) { 1 } else { 0 });
        }
        println!("");
    }
}

