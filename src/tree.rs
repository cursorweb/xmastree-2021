use crossterm::{
    cursor, execute,
    style::{Color, Stylize, Print},
};
use std::{collections::HashMap, io::Stdout};

pub enum _OutTree {
    Text,
    Rust,
}

#[derive(Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x as u16, y: y as u16 }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn render_tree(tree_file: &Vec<String>, width: usize, height: usize, base: usize) -> String {
    tree_file
        .iter()
        .enumerate()
        .map(|(i, string)| {
            let len = string.len();
            let out = if i >= height - base {
                string.clone().with(Color::DarkYellow).to_string()
            } else {
                string.clone().with(Color::DarkGreen).to_string()
            };
            let out = format!("{1:<0$}", width + out.len() - len, out);
            out
        })
        .collect::<Vec<String>>()
        .join("\r\n")
}

pub fn render_ornaments(stdout: &mut Stdout, ornaments: &HashMap<Point, Color>, offset: u16) {
    for (point, color) in ornaments {
        execute!(
            stdout,
            cursor::MoveTo(point.x, point.y + offset),
            Print("O".with(*color))
        )
        .unwrap();
    }
}
