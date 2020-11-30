use hashbrown::HashMap;
use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;

#[derive(PartialEq)]
pub enum Square {
    Empty,
    Wall,
    Goblin,
    Elf,
}

#[derive(Debug)]
pub struct BattleField {
    grid: StableGraph<(usize, usize), u32>,
    node_to_index: HashMap<(usize, usize), NodeIndex<u32>>,
    goblins: HashMap<NodeIndex<u32>, u32>,
    elves: HashMap<NodeIndex<u32>, u32>,
}

impl BattleField {
    fn read(input: &str) -> Self {
        let grid: Vec<Vec<_>> = input
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Square::Wall,
                        '.' => Square::Empty,
                        'G' => Square::Goblin,
                        'E' => Square::Elf,
                        _ => panic!("Unknown opponent"),
                    })
                    .collect()
            })
            .collect();
        let mut node_to_index = HashMap::new();
        let mut grid_graph = StableGraph::new();
        let (mut goblins, mut elves) = (HashMap::new(), HashMap::new());
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != Square::Wall {
                    node_to_index.insert((i, j), grid_graph.add_node((i, j)));
                    match grid[i][j] {
                        Square::Goblin => goblins.insert(node_to_index[&(i, j)], 200),
                        Square::Elf => elves.insert(node_to_index[&(i, j)], 200),
                        _ => None,
                    };
                }
            }
        }
        node_to_index.iter().for_each(|(index, node)| {
            let (i, j) = (index.0, index.1);
            for neighbor in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                .iter()
                .filter_map(|n| node_to_index.get(n))
            {
                grid_graph.add_edge(*node, *neighbor, 1);
                grid_graph.add_edge(*neighbor, *node, 1);
            }
        });
        BattleField {
            grid: grid_graph,
            node_to_index,
            goblins,
            elves,
        }
    }

    fn try_attack(&mut self, unit: NodeIndex<u32>, is_elf: bool) -> bool {
        let (x, y) = self.grid[unit];
        let targets_in_range: Vec<_> = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .iter()
            .filter_map(|index| self.node_to_index.get(&index))
            .filter(|node| {
                if is_elf {
                    self.goblins.contains_key(*node)
                } else {
                    self.elves.contains_key(*node)
                }
            })
            .cloned()
            .collect();
        if targets_in_range.is_empty() {
            false
        } else {
            let weakest_target = targets_in_range
                .into_iter()
                .min_by(|a, b| {
                    if is_elf {
                        (self.goblins[a], self.grid[*a]).cmp(&(self.goblins[b], self.grid[*b]))
                    } else {
                        (self.elves[a], self.grid[*a]).cmp(&(self.elves[b], self.grid[*b]))
                    }
                })
                .unwrap();
            let targets = if is_elf {
                &mut self.goblins
            } else {
                &mut self.elves
            };
            if targets[&weakest_target] <= 3 {
                targets.remove(&weakest_target);
            } else {
                *targets.entry(weakest_target).or_insert(200) -= 3;
            }
            true
        }
    }

    fn try_move(&mut self, unit: NodeIndex<u32>, is_elf: bool) -> bool {
        let closest = if is_elf { &self.goblins } else { &self.elves }
            .keys()
            .flat_map(|n| {
                let (x, y) = self.grid[*n];
                vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .into_iter()
                    .filter_map(|index| self.node_to_index.get(&index))
            })
            .filter(|n| !self.goblins.contains_key(n) && !self.elves.contains_key(n))
            .map(|n| {
                astar(
                    &self.grid.filter_map(
                        |index, node| {
                            if index == unit
                                || (!self.goblins.contains_key(&index)
                                    && !self.elves.contains_key(&index))
                            {
                                Some(node)
                            } else {
                                None
                            }
                        },
                        |_, e| Some(*e),
                    ),
                    unit,
                    |finish| finish == *n,
                    |e| *e.weight(),
                    |_| 0,
                )
            })
            .filter(|x| x.is_some())
            .map(|x| {
                let (score, path) = x.unwrap();
                (score, path[1])
            })
            .min_by(|a, b| a.cmp(b));
        match closest {
            Some((_, move_to)) => {
                if is_elf {
                    let health = self.elves.remove(&unit).unwrap();
                    self.elves.insert(move_to, health);
                } else {
                    let health = self.goblins.remove(&unit).unwrap();
                    self.goblins.insert(move_to, health);
                }
                self.try_attack(move_to, is_elf);
                true
            }
            None => false,
        }
    }

    fn turn(&mut self, unit: NodeIndex<u32>, is_elf: bool) -> bool {
        let attacked = self.try_attack(unit, is_elf);
        if !attacked {
            self.try_move(unit, is_elf)
        } else {
            true
        }
    }

    fn round(&mut self) -> bool {
        let units = self
            .goblins
            .keys()
            .chain(self.elves.keys())
            .cloned()
            .sorted_by(|a, b| self.grid[*a].cmp(&self.grid[*b]));
        let mut turns = Vec::new();
        for unit in units {
            if !self.goblins.contains_key(&unit) && !self.elves.contains_key(&unit) {
                turns.push(false)
            } else {
                let is_elf = if self.elves.contains_key(&unit) {
                    true
                } else {
                    false
                };
                if (is_elf && self.goblins.is_empty()) || (!is_elf && self.elves.is_empty()) {
                    return false;
                }
                turns.push(self.turn(unit, is_elf));
            }
        }
        true
    }
}

#[aoc(day15, part1)]
pub fn solve_day15_part1(input: &str) -> u32 {
    let mut battlefield = BattleField::read(input);
    let mut num_rounds = 0;
    loop {
        if !battlefield.round() {
            return num_rounds
                * (battlefield.elves.values().sum::<u32>()
                    + battlefield.goblins.values().sum::<u32>());
        } else {
            num_rounds += 1;
        }
    }
}
