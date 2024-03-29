use rand::prelude::*;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

use crate::{board::Board, deck::Deck};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Game {
    rng: Box<Pcg32>,
    b: Board,
    d: Deck,
    g: Vec<bool>,
    next: RefCell<Rc<Vec<u32>>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    IllegalMove,
    GameOver,
}

impl Game {
    pub fn new(boost: u32, boostpos: usize) -> Self {
        let mut rng = Pcg32::from_entropy();
        let d = Deck::new(&mut rng);
        let mut s = Self {
            rng: Box::new(rng),
            b: Board::default(),
            d: d,
            // When starting a new game with boost, the giant deck is 21 blanks.
            g: vec![false; 21],
            next: RefCell::default(),
        };
        s.b.set(boostpos, boost);
        if boost == 0 {
            // No boost, so create giant deck.
            s.new_giant();
        }
        // Deal out 8 cards into random spots; do not advance giants.
        for _ in 0..8 {
            let c = s.d.next(&mut s.rng);
            while !s.b.set(s.rng.gen_range(0..16), c) {}
        }
        s.next = RefCell::new(Rc::new(vec![s.d.next(&mut s.rng)]));
        s
    }
    /// Replaces the rng in the game with a new one and shuffles all the
    /// unknowns (deck, giant position).
    pub fn rerand(&mut self) {
        self.rng = Box::new(Pcg32::from_entropy());
        self.d.contents.shuffle(&mut self.rng);
        self.g.shuffle(&mut self.rng);
    }
    fn new_giant(&mut self) {
        self.g = vec![false; 21];
        self.g[self.rng.gen_range(0..21)] = true;
    }
    fn check_giant(&mut self) -> Option<Vec<u32>> {
        if self.g.len() == 0 {
            self.new_giant();
        }
        if !self.g.pop().unwrap() {
            return None;
        }
        let m = self.b.max_val();
        match m {
            0 | 1 | 2 | 3 | 6 | 12 | 24 => return None,
            48 => return Some(vec![6]),
            96 => return Some(vec![6, 12]),
            _ => {
                let f = self.rng.gen_range(0..=((m / 192).ilog(2)));
                let low = 6 * (2u32.pow(f));
                Some(vec![low, low * 2, low * 4])
            }
        }
    }
    fn pull(&mut self) -> u32 {
        // Determine next next.
        let next = self
            .check_giant()
            .unwrap_or_else(|| vec![self.d.next(&mut self.rng)]);
        // Set next next as next and read the old next.
        let next = self.next.replace(Rc::new(next));
        // Determine next (pick from vec).
        next[self.rng.gen_range(0..next.len())]
    }
    pub fn next(&self) -> Rc<Vec<u32>> {
        self.next.borrow().clone()
    }
    pub fn board(&self) -> Board {
        self.b
    }
    pub fn can_move(&self) -> bool {
        self.b.can_move()
    }
    fn finish(&mut self, open: Vec<usize>) -> Result<Rc<Vec<u32>>, Error> {
        if open.is_empty() {
            return Err(Error::IllegalMove);
        };
        let next = self.pull();
        self.b.set(open[self.rng.gen_range(0..open.len())], next);
        if !self.can_move() {
            return Err(Error::GameOver);
        }
        Ok(self.next())
    }
    // up/down/left/right move in the given direction and return <next> unless
    // the move was illegal or the game is over, in which case the appropriate
    // error is returned.
    pub fn up(&mut self) -> Result<Rc<Vec<u32>>, Error> {
        let res = self.b.up();
        self.finish(res)
    }
    pub fn down(&mut self) -> Result<Rc<Vec<u32>>, Error> {
        let res = self.b.down();
        self.finish(res)
    }
    pub fn left(&mut self) -> Result<Rc<Vec<u32>>, Error> {
        let res = self.b.left();
        self.finish(res)
    }
    pub fn right(&mut self) -> Result<Rc<Vec<u32>>, Error> {
        let res = self.b.right();
        self.finish(res)
    }
}
