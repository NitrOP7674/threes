use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use rand::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub(crate) struct Deck {
    contents: VecDeque<u32>,
}

impl Deck {
    // Returns the number of 1s, 2s, and 3s.
    pub fn counts(&self) -> [u32; 3] {
        self.contents.iter().fold([0; 3], |mut acc, v| {
            acc[(v - 1) as usize] = acc[(v - 1) as usize] + 1;
            acc
        })
    }
    pub(crate) fn new(rng: &mut impl Rng) -> Self {
        Self {
            contents: Self::newv(rng),
        }
    }
    fn newv(rng: &mut impl Rng) -> VecDeque<u32> {
        let mut v = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
        v.shuffle(rng);
        VecDeque::from(v)
    }
    pub(crate) fn next(&mut self, rng: &mut impl Rng) -> u32 {
        if self.contents.len() == 0 {
            self.contents = Self::newv(rng);
        }
        VecDeque::pop_back(&mut self.contents).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::deck::Deck;

    #[test]
    fn next() {
        let mut d = Deck::new(&mut thread_rng());
        let mut got = Vec::new();
        for _ in 0..12 {
            got.push(d.next(&mut thread_rng()));
        }
        assert_eq!(got.iter().filter(|&x| *x == 1).count(), 4);
        assert_eq!(got.iter().filter(|&x| *x == 2).count(), 4);
        assert_eq!(got.iter().filter(|&x| *x == 3).count(), 4);
        assert_eq!(got.len(), 12);

        got.clear();
        for _ in 0..12 {
            got.push(d.next(&mut thread_rng()));
        }
        assert_eq!(got.iter().filter(|&x| *x == 1).count(), 4);
        assert_eq!(got.iter().filter(|&x| *x == 2).count(), 4);
        assert_eq!(got.iter().filter(|&x| *x == 3).count(), 4);
        assert_eq!(got.len(), 12);
    }
}
