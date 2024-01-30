use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    style::{self, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rand::prelude::*;
use threes::game::{self, Game};

use std::io::{self, stdout, Write};

fn printone(n: u32) -> io::Result<()> {
    match n {
        0 => {
            stdout().execute(style::PrintStyledContent(
                " 0 ".with(style::Color::White).on(style::Color::Black),
            ))?;
        }
        1 => {
            stdout().execute(style::PrintStyledContent(
                " 1 ".with(style::Color::White).on(style::Color::Blue),
            ))?;
        }
        2 => {
            stdout().execute(style::PrintStyledContent(
                " 2 ".with(style::Color::White).on(style::Color::Red),
            ))?;
        }
        n => {
            let p = if n < 10 {
                format!(" {} ", n)
                    .with(style::Color::Black)
                    .on(style::Color::Grey)
            } else if n < 100 {
                format!(" {}", n)
                    .with(style::Color::Black)
                    .on(style::Color::Grey)
            } else {
                format!("{}", n)
                    .with(style::Color::Black)
                    .on(style::Color::Grey)
            };
            stdout().execute(style::PrintStyledContent(p))?;
        }
    };
    Ok(())
}

fn printboard(g: &Game, mv: Option<Move>, best: i32) -> io::Result<()> {
    stdout().execute(cursor::MoveTo(1, 1))?;
    write!(stdout(), "Next: ")?;
    match g.next().as_slice() {
        [n] => printone(*n)?,
        [n @ ..] => {
            stdout().execute(style::PrintStyledContent(
                format!("{:?}", n)
                    .with(style::Color::Black)
                    .on(style::Color::Grey),
            ))?;
            ()
        }
    };

    stdout().execute(cursor::MoveTo(1, 2))?;
    write!(stdout(), "----------------")?;

    let b = g.board().0;
    for row in 0..4 {
        stdout().execute(cursor::MoveTo(1, 3 + row))?;
        for col in 0..4 {
            let n = b[(row * 4 + col) as usize];
            if n < 1000 {
                print!(" ");
            }
            printone(n)?;
        }
    }
    if mv.is_some() {
        stdout().execute(cursor::MoveTo(1, 8))?;
        write!(stdout(), "Moved {:?} ({})", mv.unwrap(), best)?;
    }
    stdout().execute(cursor::MoveTo(1, 10))?;
    write!(stdout(), "Esc or 'q' to exit.")?;
    stdout().execute(cursor::MoveTo(1, 11))?;
    Ok(())
}

#[derive(Copy, Debug, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

const ITERS: i32 = 10000;

fn score(g: Game, mv: Move) -> i32 {
    let mut avg = 0;
    let mut best = 0;
    let mut worst = 9999;
    let backup = g.clone();
    for i in 0..ITERS {
        let mut g = backup.clone();
        g.rerand();
        let res = match mv {
            Move::Up => g.up(),
            Move::Down => g.down(),
            Move::Left => g.left(),
            Move::Right => g.right(),
        };
        match res {
            Err(game::Error::IllegalMove) => {
                return 0;
            }
            Err(game::Error::GameOver) => {
                avg += 1;
                continue;
            }
            _ => {}
        }
        let c = run(&mut g);
        if i % 100 == 50 {
            stdout().execute(cursor::MoveTo(15, 12)).ok();
            write!(stdout(), "score={}...", avg / i + worst / 2 + best / 2).ok();
        }
        avg += c;
        if c > best {
            best = c;
        }
        if c < worst {
            worst = c;
        }
    }
    avg / ITERS + worst * 10 + best
}

fn run(g: &mut Game) -> i32 {
    let mut c = 0;
    loop {
        let mv = thread_rng().gen_range(0..4);
        let res = match mv {
            0 => g.up(),
            1 => g.down(),
            2 => g.left(),
            _ => g.right(),
        };
        if res == Err(game::Error::GameOver) {
            return c;
        }
        if res.is_ok() {
            c += 1;
        }
    }
}

fn main() -> io::Result<()> {
    let mut g = Game::new(192, 12);
    crossterm::terminal::enable_raw_mode()?;
    stdout().execute(crossterm::cursor::Hide)?;

    let mut mv: Option<Move> = None;
    let mut best: i32 = 0;
    'outer: loop {
        stdout().execute(Clear(ClearType::All))?;
        printboard(&g, mv, best)?;
        if !g.can_move() {
            break;
        }
        mv = None;
        best = 0;
        let sc = score(g.clone(), Move::Up);
        stdout().execute(cursor::MoveTo(1, 12))?;
        write!(stdout(), "UP: {}", sc)?;
        if sc > best {
            best = sc;
            mv = Some(Move::Up);
        }
        let sc = score(g.clone(), Move::Down);
        stdout().execute(cursor::MoveTo(1, 13))?;
        write!(stdout(), "DOWN: {}", sc)?;
        if sc > best {
            best = sc;
            mv = Some(Move::Down);
        }
        let sc = score(g.clone(), Move::Left);
        stdout().execute(cursor::MoveTo(1, 14))?;
        write!(stdout(), "LEFT: {}", sc)?;
        if sc > best {
            best = sc;
            mv = Some(Move::Left);
        }
        let sc = score(g.clone(), Move::Right);
        stdout().execute(cursor::MoveTo(1, 15))?;
        write!(stdout(), "RIGHT: {}", sc)?;
        stdout().execute(cursor::MoveTo(1, 16))?;
        if sc > best {
            best = sc;
            mv = Some(Move::Right);
        } /*
          loop {
              match read()? {
                  Event::Key(KeyEvent {
                      code: KeyCode::Esc,
                      kind: KeyEventKind::Press,
                      ..
                  })
                  | Event::Key(KeyEvent {
                      code: KeyCode::Char('q'),
                      kind: KeyEventKind::Press,
                      ..
                  }) => {
                      break 'outer;
                  }
                  Event::Key(KeyEvent {
                      kind: KeyEventKind::Press,
                      ..
                  }) => {}
                  _ => {
                      break;
                  }
              }
          }
          */
        match mv {
            Some(Move::Up) => g.up().ok(),
            Some(Move::Down) => g.down().ok(),
            Some(Move::Left) => g.left().ok(),
            Some(Move::Right) => g.right().ok(),
            None => unreachable!(),
        };
    }
    stdout().execute(crossterm::style::ResetColor)?;
    stdout().execute(crossterm::cursor::Show)?;
    crossterm::terminal::disable_raw_mode()?;
    println!("\nexited");
    Ok(())
}
