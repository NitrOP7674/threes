use std::collections::VecDeque;

use rand::prelude::*;

#[derive(Debug)]
pub(crate) struct Deck {
    contents: VecDeque<u32>,
}

impl Deck {
    pub(crate) fn new() -> Self {
        Self {
            contents: Self::newv(),
        }
    }
    fn newv() -> VecDeque<u32> {
        let mut v = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
        let mut rng = thread_rng();
        v.shuffle(&mut rng);
        VecDeque::from(v)
    }
    pub(crate) fn next(&mut self) -> u32 {
        if self.contents.len() == 0 {
            self.contents = Self::newv();
        }
        VecDeque::pop_back(&mut self.contents).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::Deck;

    #[test]
    fn next() {
        let mut d = Deck::new();
        let mut got = Vec::new();
        for _ in 0..12 {
            got.push(d.next());
        }
        assert_eq!(got.iter().filter(|&x| *x == 1).count(), 4);
        assert_eq!(got.iter().filter(|&x| *x == 2).count(), 4);
        assert_eq!(got.iter().filter(|&x| *x == 3).count(), 4);
        assert_eq!(got.len(), 12);

        got.clear();
        for _ in 0..12 {
            got.push(d.next());
        }
        assert_eq!(got.iter().filter(|&x| *x == 1).count(), 4);
        assert_eq!(got.iter().filter(|&x| *x == 2).count(), 4);
        assert_eq!(got.iter().filter(|&x| *x == 3).count(), 4);
        assert_eq!(got.len(), 12);
    }
}
