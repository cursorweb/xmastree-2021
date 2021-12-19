use clap::{App, Arg};
use std::{process::exit, fs};


// config
const DEFAULT_TREE: &str = include_str!("./tree.txt");
const NAME: &str = "XMASTREE2021";
const VERSION: &str = "0.1.0";
const AUTHOR: &str = "Junhao \"Jerry\" Zhang";
const ABOUT: &str = "Generate beautiful trees!";

pub struct ParseOut {
    pub tree_file: Vec<String>,
    pub tlen: usize,
    pub base: usize
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
                .help("The file all output will be put into"),
        )
        .arg(
            Arg::with_name("type")
                .short("f")
                .takes_value(true)
                .help("The output type, 'rs' or 'text'"),
        )
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
                    custom_tree = false;
                    eprintln!("Error reading file {}: {}\nUsing the default tree model instead!", str, error);
                    String::from(DEFAULT_TREE)
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

    ParseOut { 
        tree_file, base, tlen
    }
}