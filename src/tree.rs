use crossterm::style::{Color, Stylize};

pub enum _OutTree {
    Text,
    Rust,
}

#[derive(Eq, PartialOrd, Ord)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point {
            x, y
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn render_tree(tree_file: &Vec<String>, width: usize, height: usize, base: usize) -> String {
    tree_file.iter().enumerate().map(|(i, string)| {
        let len = string.len();
        let out = if i >= height - base {
            string.clone().with(Color::DarkYellow).to_string()
        } else {
            string.clone()
        };
        let out = format!("{1:<0$}", width + out.len() - len, out);
        out
    }).collect::<Vec<String>>().join("\n")
}