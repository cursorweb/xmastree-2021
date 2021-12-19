use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

mod parse_args;
use parse_args::{parse_args, ParseOut};

mod tree;
use tree::{render_ornaments, render_tree, Point};

use std::{collections::HashMap, io::stdout};

fn main() {
    let ParseOut {
        tree_file,
        tlen,
        base,
    } = parse_args();

    let width = tree_file.iter().fold(tree_file[0].len(), |prev, curr| {
        if prev < curr.len() {
            curr.len()
        } else {
            prev
        }
    }) as i32;
    let height = tlen as i32;

    let mut x = 0i32;
    let mut y = 0i32;

    // ornaments
    let mut orn: HashMap<Point, Color> = HashMap::new();
    let colors = [
        Color::Blue,
        Color::Cyan,
        Color::DarkMagenta,
        Color::White,
        Color::Black,
        Color::Yellow,
        Color::Red,
        Color::Magenta,
        Color::DarkBlue,
    ];
    let mut curr_color = 0;

    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0),
        Print(render_tree(
            &tree_file,
            width as usize,
            height as usize,
            base
        )),
        cursor::MoveTo(0, 0),
        Print('X'.with(colors[curr_color]))
    )
    .unwrap();

    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::NONE,
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
            }) => y -= 1,

            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::NONE,
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
            }) => x -= 1,

            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::NONE,
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
            }) => y += 1,

            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::NONE,
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
            }) => x += 1,

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            }) => {
                orn.insert(Point::new(x, y), colors[curr_color]);
            }

            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            }) => {
                orn.remove(&Point::new(x, y));
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::NONE,
            }) => {
                if curr_color == 0 {
                    curr_color = colors.len() - 1;
                } else {
                    curr_color -= 1;
                }
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('r'),
                modifiers: KeyModifiers::NONE,
            }) => {
                if curr_color == colors.len() - 1 {
                    curr_color = 0;
                } else {
                    curr_color += 1;
                }
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
            }) => {
                execute!(
                    stdout,
                    Clear(ClearType::All),
                    cursor::Show,
                    cursor::MoveTo(0, 0)
                )
                .unwrap();

                // print
                execute!(
                    stdout,
                    cursor::MoveTo(0, 1),
                    Print(render_tree(
                        &tree_file,
                        width as usize,
                        height as usize,
                        base
                    ))
                )
                .unwrap();
                render_ornaments(&mut stdout, &orn);
                execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
                break;
            }

            _ => (),
        }

        if x < 0 {
            x = width - 1 as i32;
        }
        if x >= width {
            x = 0;
        }
        if y < 0 {
            y = height - 1 as i32;
        }
        if y >= height {
            y = 0;
        }

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Print(render_tree(
                &tree_file,
                width as usize,
                height as usize,
                base
            ))
        )
        .unwrap();

        render_ornaments(&mut stdout, &orn);

        execute!(
            stdout,
            cursor::MoveTo(x as u16, y as u16),
            Print('X'.with(colors[curr_color]))
        )
        .unwrap();
    }

    //disabling raw mode
    disable_raw_mode().unwrap();
}
