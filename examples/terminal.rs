use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    style::{self, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use threes::game::Game;

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

fn printboard(g: &Game) -> io::Result<()> {
    stdout()
        .execute(Clear(ClearType::All))?
        .execute(cursor::MoveTo(1, 1))?;
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
    stdout().execute(cursor::MoveTo(1, 10))?;
    write!(stdout(), "Arrows to move; 'b' to undo; Esc or 'q' to exit.")?;
    stdout().execute(cursor::MoveTo(1, 11))?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut g = Game::new();
    let mut old_g = g.clone();
    crossterm::terminal::enable_raw_mode()?;
    stdout().execute(crossterm::cursor::Hide)?;

    loop {
        printboard(&g)?;
        if !g.can_move() {
            break;
        }
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                kind: KeyEventKind::Press,
                ..
            }) => {
                old_g = g.clone();
                let _ = g.up();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                ..
            }) => {
                old_g = g.clone();
                let _ = g.down();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                ..
            }) => {
                old_g = g.clone();
                let _ = g.left();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                ..
            }) => {
                old_g = g.clone();
                let _ = g.right();
            }
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
                break;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('b'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                g = old_g.clone();
            }
            _ => {}
        }
    }
    stdout().execute(crossterm::style::ResetColor)?;
    stdout().execute(crossterm::cursor::Show)?;
    crossterm::terminal::disable_raw_mode()?;
    println!("\nexited");
    Ok(())
}
