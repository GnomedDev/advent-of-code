use std::iter::Peekable;

use aoc_runner_derive::{aoc, aoc_generator};

type Directory<'a> = std::collections::HashMap<&'a str, Node<'a>>;

#[derive(Debug)]
pub enum Node<'a> {
    Dir(Directory<'a>),
    File { size: u64 },
}

fn parse_tree<'root, 'input>(
    lines: &'root mut Peekable<impl Iterator<Item = &'input str>>,
) -> Directory<'input> {
    let mut root_dir = Directory::new();
    let mut current_dir: &mut Directory = &mut root_dir;
    let mut path = Vec::new();

    loop {
        let Some(line) = lines.next() else {break};
        let line = line
            .strip_prefix("$ ")
            .expect("shouldn't be listing files yet");

        match line.strip_prefix("cd ") {
            Some(dir_name) => {
                if dir_name == ".." {
                    current_dir = {
                        path.remove(path.len() - 1);
                        let mut last_dir = &mut root_dir;
                        for dir_name in &path {
                            last_dir = match last_dir.get_mut(dir_name).unwrap() {
                                Node::File { .. } => panic!("Cannot cd into a file!"),
                                Node::Dir(dir) => dir,
                            };
                        }

                        last_dir
                    }
                } else {
                    path.push(dir_name);
                    current_dir = match current_dir.get_mut(dir_name).unwrap() {
                        Node::File { .. } => panic!("Cannot cd into a file!"),
                        Node::Dir(dir) => dir,
                    }
                }
            }
            None => {
                assert_eq!(line, "ls");
                while let Some(line) = lines.peek() {
                    let line = if line.starts_with("$ ") {
                        break;
                    } else {
                        lines.next().unwrap()
                    };

                    let (size, name) = line.split_once(' ').unwrap();
                    let node = match size.parse::<u64>() {
                        Ok(size) => Node::File { size },
                        Err(_) => Node::Dir(Directory::new()),
                    };

                    current_dir.insert(name, node);
                }
            }
        };
    }

    root_dir
}

fn walk_tree_prune<'input>(total_size: &mut u64, node: &mut Node<'input>) -> u64 {
    match node {
        Node::File { size } => *size,
        Node::Dir(dir) => {
            let size: u64 = dir
                .values_mut()
                .map(|d| walk_tree_prune(total_size, d))
                .sum();

            if size < 100_000 {
                *total_size += size
            }

            size
        }
    }
}

#[aoc(day7, part1)]
pub fn get_prune_size(input: &str) -> u64 {
    let dir_tree = parse_tree(&mut input.lines().skip(1).peekable());
    let mut root_node = Node::Dir(dir_tree);

    let mut total_size = 0;
    walk_tree_prune(&mut total_size, &mut root_node);
    total_size
}
