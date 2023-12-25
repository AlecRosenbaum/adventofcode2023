use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::collections::HashMap;

type MapEntry = (String, u8);

const NUM_MAP: [(&str, u8); 20] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("zero", 0),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn get_num_map() -> Vec<MapEntry> {
    NUM_MAP.iter().map(|e| (e.0.to_string(), e.1)).collect()
}

#[derive(Debug)]
struct Tree {
    nodes: HashMap<char, Node>
}

#[derive(Debug)]
enum Node {
    Branch(Tree),
    Leaf(u8),
}

impl Tree {
    fn add(&mut self, rem: &String, value: u8) {
        let next_char = rem.chars().next().expect("No next char");
        if rem.len() > 1 {
            let node: &mut Node;
            match self.nodes.get_mut(&next_char) {
                Some(next) => node = next,
                None => {
                    self.nodes.insert(
                        next_char,
                        Node::Branch(Tree { nodes: HashMap::new(), })
                    );
                    node = self.nodes.get_mut(&next_char).unwrap();
                }
            }
            match node {
                Node::Leaf(_) => panic!("No dual leaf branches!"),
                Node::Branch(ref mut subtree) => subtree.add(&rem[1..].to_string(), value),
            }
        } else {
            self.nodes.insert(next_char, Node::Leaf(value));
        }
    }

    fn get(&self, val: &str) -> Option<u8> {
        let first_char = val.chars().next()?;
        let node = self.nodes.get(&first_char)?;
        match node {
            Node::Branch(subtree) => subtree.get(&val[1..]),
            Node::Leaf(val) => Some(*val),
        }
    }
}


fn read_input_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("Failed to open file");
    BufReader::new(file).lines().map(|e| e.unwrap()).collect()
}

fn gen_tree(map: &[MapEntry]) -> Tree {
    let mut tree = Tree { nodes: HashMap::new() };
    for elem in map {
        tree.add(&elem.0, elem.1);
    };
    tree
}

fn reverse_num_map(map: &[MapEntry]) -> Vec<MapEntry> {
    map.iter().map(|entry| (entry.0.chars().rev().collect(), entry.1)).collect()
}

fn line_val(line: &String, forward_tree: &Tree, rev_tree: &Tree) -> u32 {
    let first: u8 = {
        let mut line_val: Option<u8> = None;
        for i in 0..line.len() {
            let sub_line: &str = &line[i..];
            line_val = forward_tree.get(sub_line);
            match line_val {
                Some(_val) => break,
                _ => (),
            };
        }
        line_val.expect("Did not find line value!")
    };
    let last: u8 = {
        let mut line_val: Option<u8> = None;
        let rev_line: String = line.chars().rev().collect();
        for i in 0..rev_line.len() {
            let sub_line: &str = &rev_line[i..];
            line_val = rev_tree.get(sub_line);
            match line_val {
                Some(_val) => break,
                _ => (),
            };
        }
        line_val.expect("Did not find rev line value!")
    };
    (first * 10 + last).into()
}

fn main() -> std::io::Result<()> {
    let num_map: Vec<MapEntry> = get_num_map();
    let tree = gen_tree(&num_map);
    let rev_tree = gen_tree(&reverse_num_map(&num_map));

    let contents = read_input_lines();

    let sum = contents.iter().map(|line| line_val(line, &tree, &rev_tree)).fold(0, |acc, x| acc + x);

//     let contents: Vec<String> = vec![
//         "two1nine",
//         "eightwothree",
//         "abcone2threexyz",
//         "xtwone3four",
//         "4nineeightseven2",
//         "zoneight234",
//         "7pqrstsixteen",
//     ].iter().map(|e| e.to_string()).collect();

//     for line in contents.iter() {
//         println!("{}", line_val(line, &tree, &rev_tree));
//     }
    println!("{}", sum);
    Ok(())
}
