use crate::{parse_args::ParseOut, tree::Point};

use crossterm::style::{Color, Stylize};

use std::{collections::HashMap, fs};

pub enum OutType {
    Text(String),
    Rust(String),
    None,
}

impl ParseOut {
    pub fn render(&self, t: &OutType, orns: &HashMap<Point, Color>) {
        match t {
            OutType::Text(file) => {
                fs::write(file, self.render_tree(&self.tree_file, orns)).unwrap();
            }
            OutType::Rust(file) => {
                let text = include_str!("txt/rust.txt");
                fs::write(
                    file,
                    text.replace("tree", &self.render_tree(&self.tree_file, orns)),
                )
                .unwrap();
            }
            _ => (),
        }
    }

    fn render_tree(&self, tree_file: &Vec<String>, orns: &HashMap<Point, Color>) -> String {
        tree_file
            .iter()
            .enumerate()
            .map(|(y, string)| {
                string
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if orns.contains_key(&Point::new(x as i32, y as i32)) {
                            'O'.with(orns[&Point::new(x as i32, y as i32)]).to_string()
                        } else {
                            if y >= self.tlen - self.base {
                                c.with(Color::DarkYellow).to_string()
                            } else {
                                c.with(Color::DarkGreen).to_string()
                            }
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}
