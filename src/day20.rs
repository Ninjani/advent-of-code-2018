use hashbrown::HashMap;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use petgraph::Undirected;

pub fn solve_day20_part1(input: &str) {
    let mut map = Graph::new_undirected();
}

#[derive(Clone, Copy)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn next(&self, direction: char) -> Coordinate {
        match direction {
            'N' => Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            'S' => Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            'E' => Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            'W' => Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            _ => panic!("Unknown direction"),
        }
    }
}

pub struct Map {
    map: Graph<Coordinate, (), Undirected>,
    node_to_index: HashMap<Coordinate, NodeIndex<u32>>,
    current: Coordinate,
    previous: Option<Coordinate>,
    paren_level: usize,
    attached: bool,
}

impl Map {
    pub fn new() -> Self {
        Map {
            map: Graph::new_undirected(),
            node_to_index: HashMap::new(),
            current: Coordinate { x: 0, y: 0 },
            previous: None,
            paren_level: 0,
            attached: false,
        }
    }

    pub fn get_node_index(&mut self, coordinate: Coordinate) -> NodeIndex<u32> {
        if !self.node_to_index.contains_key(&coordinate) {
            self.node_to_index
                .insert(coordinate, self.map.add_node(coordinate));
        }
        self.node_to_index[&coordinate]
    }

    pub fn create(&mut self, path: &str) -> Option<Coordinate> {
        let mut path_chars = input.chars();
        while let Some(ch) = path_chars.next() {
            if "NSEW".contains(ch) {
                self.map.add_edge(
                    self.get_node_index(self.current),
                    self.get_node_index(self.current.next(ch)),
                    (),
                );
                self.current = self.current.next(ch);
                if !self.attached {
                    if let Some(previous) = self.previous {
                        self.map.add_edge(
                            self.get_node_index(self.current),
                            self.get_node_index(previous),
                            (),
                        );
                        self.attached = true;
                    }
                }
            }
            match ch {
                '^' => (),
                '(' => {
                    self.paren_level += 1;
                    self.previous = Some(self.current);
                }
                '|' => {
                    self.attached = false;
                    if let Some(previous) = self.previous {
                        self.current = previous;
                    }
                }
                ')' => {
                    self.paren_level -= 1;
                    if let Some(previous) = self.previous {
                        self.current = previous;
                    }
                }
                '$' => {
                    break;
                }
                _ => panic!("Unrecognized character"),
            }
        }
        None
    }
}
