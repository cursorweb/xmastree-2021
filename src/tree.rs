use crossterm::style::{Color, Stylize};

pub enum _OutTree {
    Text,
    Rust,
}

#[derive(Eq, PartialOrd, Ord)]
pub struct Point {
    x: u16,
    y: u16,
    c: Color
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point {
            x, y, c: Color::Black
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn render_tree(tree_file: &Vec<String>, width: usize) -> String {
    tree_file.iter().map(|string| {
        format!("{1:<0$}", width, string)
    }).collect::<Vec<String>>().join("\n")
}