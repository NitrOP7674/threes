use std::collections::VecDeque;

use rand::prelude::*;

use crate::{board::Board, deck::Deck};

#[derive(Debug)]
struct Game {
    b: Board,
    d: Deck,
    g: VecDeque<bool>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            b: Board::default(),
            d: Deck::new(),
            g: Self::newgiant(),
        }
    }
    fn newgiant() -> VecDeque<bool> {
        let mut v = vec![false; 21];
        v[thread_rng().gen::<usize>() % 21] = true;
        VecDeque::from(v)
    }
}
