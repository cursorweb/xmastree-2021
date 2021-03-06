use clap::{App, Arg};
use std::{process::exit, fs};

use colorful::Colorful;

use super::gen::OutType;


// config
const DEFAULT_TREE: &str = include_str!("txt/tree.txt");
const NAME: &str = "XMASTREE2021";
const VERSION: &str = "0.1.0";
const AUTHOR: &str = "Junhao \"Jerry\" Zhang";
const ABOUT: &str = "Generate beautiful trees!";

pub struct ParseOut {
    pub tree_file: Vec<String>,
    pub tlen: usize,
    pub base: usize,
    pub out_type: OutType,
}

pub fn parse_args() -> ParseOut {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(
            Arg::with_name("tree")
                .short("t")
                .takes_value(true)
                .help("The tree file to use"),
        )
        .arg(
            Arg::with_name("base")
                .short("b")
                .takes_value(true)
                .help("How many lines from the last line is the base of the tree"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .help("The file all output will be put into. Default: 'tree.out'"),
        )
        .arg(
            Arg::with_name("type")
                .short("f")
                .takes_value(true)
                .help("The output type, 'rs'/'rust', 'txt'/'text', or 'o'/'none'. Default: 'none'"),
        )
        .after_help::<&str>(format!("{}
The {} marker indicates your cursor.
Use the {} to move the cursor! It is colored based on the selected color.
Use {} to cycle to the previous color and {} to cycle to the next color.
Press {} to place the ornament down.
Press {} to remove an ornament the marker is on.
Press {} to reset the entire tree.
When you are done, press {} to generate the tree!", "EDITOR USAGE".bold().underlined(), "X".blue(), "ARROW KEYS".blue(), "e".blue(), "r".blue(), "enter".blue(), "esc".blue(), "ctrl".blue(), "q".blue()).as_str())
        .get_matches();
    
    let custom_tree;

    let tree_file: Vec<String> = match matches.value_of("tree") {
        Some(str) => {
            match fs::read_to_string(str) {
                Ok(string) => {
                    custom_tree = true;
                    string
                },
                Err(error) => {
                    eprintln!("Error reading file {}: {}", str, error);
                    exit(1);
                }
            }
        },
        None => {
            custom_tree = false;
            String::from(DEFAULT_TREE)
        }
    }.lines().map(|x| x.into()).collect();
    let tlen = tree_file.len();

    let base: usize = match matches.value_of("base") {
        Some(v) => match v.parse() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to parse number: {}", e);
                exit(1);
            }
        },
        None => {
            if custom_tree {
                eprintln!("If you are using a custom tree, you should specify a base!\n
The base is the 'stump' of the tree.");
                0
            } else {
                5
            }
        }
    };

    let file: String = match matches.value_of("output") {
        Some(x) => x.into(),
        None => "tree.out".into()
    };

    let out_type = match matches.value_of("type") {
        Some(v) => {
            if v == "rs" || v == "rust" {
                OutType::Rust(file.to_string())
            }
            else if v == "txt" || v == "text" {
                OutType::Text(file.to_string())
            }
            else if v == "o" || v == "none" {
                OutType::None
            }
            else {
                eprintln!("Unknown out type {}!", v);
                exit(1);
            }
        },
        None => {
            OutType::None
        }
    };

    ParseOut { 
        tree_file, base, tlen, out_type
    }
}