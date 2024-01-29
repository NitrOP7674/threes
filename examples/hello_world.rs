extern crate threes;

use threes::game::Game;

fn main() {
    let mut g = Game::new();
    loop {
        println!("board:\n{}", g.board());
        println!("next: {:?}", g.next());
        if g.down().is_ok() {
            println!("down");
            continue;
        }
        if g.left().is_ok() {
            println!("left");
            continue;
        }
        if g.right().is_ok() {
            println!("right");
            continue;
        }
        if g.up().is_ok() {
            println!("up");
            continue;
        }
        return;
    }
}
