use serde::{Deserialize, Serialize};

/// A board for threes, holding the Board currently in play and allowing its
/// manipulation.  Board::default() provides an empty board.
#[derive(Copy, Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Board([u32; 16], u32);

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}\n{:?}\n{:?}\n{:?}",
            &self.0[0..4],
            &self.0[4..8],
            &self.0[8..12],
            &self.0[12..16]
        )
    }
}

// board layout:
// [  0,  1,  2,  3,
//    4,  5,  6,  7,
//    8,  9, 10, 11,
//   12, 13, 14, 15]
const fn rev1(x: [usize; 4]) -> [usize; 4] {
    [x[3], x[2], x[1], x[0]]
}
const fn rev(x: [[usize; 4]; 4]) -> [[usize; 4]; 4] {
    [rev1(x[0]), rev1(x[1]), rev1(x[2]), rev1(x[3])]
}

const LEFTS: [[usize; 4]; 4] = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
const RIGHTS: [[usize; 4]; 4] = rev(LEFTS);
const UPS: [[usize; 4]; 4] = [[0, 4, 8, 12], [1, 5, 9, 13], [2, 6, 10, 14], [3, 7, 11, 15]];
const DOWNS: [[usize; 4]; 4] = rev(UPS);

fn combines(a: u32, b: u32) -> bool {
    (a == 1 && b == 2) || (a == 2 && b == 1) || (a != 1 && a != 2 && a == b)
}

impl Board {
    pub fn can_move(&self) -> bool {
        for i in 0..16 {
            if self.0[i] == 0 {
                return true;
            }
        }
        for i in (0..16).step_by(4) {
            for j in i..i + 3 {
                if combines(self.0[j], self.0[j + 1]) {
                    return true;
                }
            }
        }
        for i in 0..4 {
            for j in (i..i + 12).step_by(4) {
                if combines(self.0[j], self.0[j + 4]) {
                    return true;
                }
            }
        }
        false
    }
    pub(crate) fn left(&mut self) -> Vec<usize> {
        LEFTS.into_iter().filter_map(|x| self.squish(&x)).collect()
    }
    pub(crate) fn right(&mut self) -> Vec<usize> {
        RIGHTS.into_iter().filter_map(|x| self.squish(&x)).collect()
    }
    pub(crate) fn up(&mut self) -> Vec<usize> {
        UPS.into_iter().filter_map(|x| self.squish(&x)).collect()
    }
    pub(crate) fn down(&mut self) -> Vec<usize> {
        DOWNS.into_iter().filter_map(|x| self.squish(&x)).collect()
    }
    pub(crate) fn max_val(&self) -> u32 {
        self.1
    }
    // Sets [pos] to c if it is empty and returns true if successful.
    pub(crate) fn set(&mut self, pos: usize, c: u32) -> bool {
        if pos < 16 && self.0[pos] == 0 {
            self.0[pos] = c;
            return true;
        }
        false
    }
    pub(crate) fn board(&self) -> [u32; 16] {
        self.0
    }

    // Squish the elements described by the array to the left.  Returns
    // Some(x[3]) if the items were shifted or None if not.
    fn squish(&mut self, x: &[usize; 4]) -> Option<usize> {
        let mut shift = false;
        for i in 0..3 {
            let idx = x[i];
            let idxp1 = x[i + 1];
            if self.0[idx] == 0 || shift {
                self.0[idx] = self.0[idxp1];
                shift = true;
            } else if combines(self.0[idx], self.0[idxp1]) {
                let v = self.0[idx] + self.0[idxp1];
                if v > self.1 {
                    self.1 = v;
                }
                self.0[idx] = v;
                shift = true;
            }
        }
        if shift {
            self.0[x[3]] = 0;
            return Some(x[3]);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn left() {
        let mut b = Board([3, 0, 1, 2, 6, 6, 1, 2, 12, 6, 3, 1, 0, 3, 6, 6], 12);
        b.left();

        let want = Board([3, 1, 2, 0, 12, 1, 2, 0, 12, 6, 3, 1, 3, 6, 6, 0], 12);
        assert_eq!(b, want);
        assert!(b.can_move());
    }
    #[test]
    fn right() {
        let mut b = Board([3, 0, 1, 2, 6, 6, 1, 3, 12, 6, 3, 1, 0, 3, 6, 6], 12);
        b.right();

        let want = Board([0, 3, 0, 3, 0, 12, 1, 3, 12, 6, 3, 1, 0, 0, 3, 12], 12);
        assert_eq!(b, want);
        assert!(b.can_move());
    }
    #[test]
    fn up() {
        let mut b = Board([3, 0, 1, 2, 6, 6, 1, 1, 12, 6, 3, 3, 12, 3, 6, 6], 12);
        b.up();

        let want = Board([3, 6, 1, 3, 6, 6, 1, 3, 24, 3, 3, 6, 0, 0, 6, 0], 24);
        assert_eq!(b, want);
        assert!(b.can_move());
    }
    #[test]
    fn down() {
        let mut b = Board([3, 0, 1, 2, 6, 6, 1, 1, 12, 6, 3, 3, 12, 3, 6, 6], 12);
        b.down();

        let want = Board([0, 0, 1, 0, 3, 0, 1, 3, 6, 12, 3, 3, 24, 3, 6, 6], 24);
        assert_eq!(b, want);
        assert!(b.can_move());
    }
    #[test]
    fn can_move() {
        let b = Board([3, 1, 3, 1, 2, 3, 2, 3, 3, 1, 3, 1, 2, 3, 2, 3], 3);
        assert!(!b.can_move());
    }
}
