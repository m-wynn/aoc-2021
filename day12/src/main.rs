use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    rc::Rc,
};

aoc::main!(12);

#[derive(Default)]
pub struct Day12 {}

impl aoc::AoCSolution for Day12 {
    type ConvertedType = NodeCell;
    type ReturnType = usize;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        let mut nodes: HashMap<String, NodeCell> = HashMap::new();
        let node_typer = |name| -> NodeType {
            match name {
                "start" => NodeType::Start,
                "end" => NodeType::End,
                x if x.chars().next().unwrap().is_uppercase() => NodeType::Big,
                x if x.chars().next().unwrap().is_lowercase() => NodeType::Small,
                _ => panic!("Unknown node type"),
            }
        };

        input.lines().for_each(|x| {
            let (left, right) = x.split_once('-').unwrap();
            let left_node = nodes
                .entry(left.to_string())
                .or_insert_with(|| {
                    Rc::new(RefCell::new(Node {
                        name: left.to_string(),
                        connections: Vec::new(),
                        node_type: node_typer(left),
                    }))
                })
                .clone();
            let right_node = nodes
                .entry(right.to_string())
                .or_insert_with(|| {
                    Rc::new(RefCell::new(Node {
                        name: right.to_string(),
                        connections: Vec::new(),
                        node_type: node_typer(right),
                    }))
                })
                .clone();
            left_node.borrow_mut().connections.push(right_node.clone());
            right_node.borrow_mut().connections.push(left_node);
        });
        nodes.remove("start").unwrap()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let set = HashSet::new();
        search(input, set, true).unwrap()
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let set = HashSet::new();
        search(input, set, false).unwrap()
    }
}

pub type NodeCell = Rc<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
    name: String,
    connections: Vec<NodeCell>,
    node_type: NodeType,
}
impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Hash, PartialEq)]
pub enum NodeType {
    Big,
    Small,
    Start,
    End,
}

fn search(
    current: &NodeCell,
    visited: HashSet<String>,
    allow_small_cave_revisit: bool,
) -> Option<usize> {
    let mut visited = visited;
    let mut allow_small_cave_revisit = allow_small_cave_revisit;

    if current.borrow().node_type == NodeType::End {
        return Some(1);
    }
    if visited.contains(&current.borrow().name) {
        if allow_small_cave_revisit || current.borrow().node_type == NodeType::Start {
            return None;
        }
        allow_small_cave_revisit = true;
    }
    if current.borrow().node_type == NodeType::Small
        || current.borrow().node_type == NodeType::Start
    {
        visited.insert(current.borrow().name.clone());
    }
    Some(
        current
            .borrow()
            .connections
            .iter()
            .filter_map(|nextnode| search(nextnode, visited.clone(), allow_small_cave_revisit))
            .sum(),
    )
}
