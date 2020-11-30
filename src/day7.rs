use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use petgraph::stable_graph::StableGraph;
use petgraph::Direction;

pub fn generate_day7(input: &str) -> StableGraph<char, ()> {
    let (mut graph, mut node_to_index) = (StableGraph::new(), HashMap::new());
    let (mut index_1, mut index_2);
    for line in input.trim().split('\n') {
        let parts: Vec<_> = line.split_whitespace().collect();
        let (c1, c2) = (
            parts[1].chars().next().unwrap(),
            parts[7].chars().next().unwrap(),
        );
        index_1 = *node_to_index
            .entry(c1)
            .or_insert_with(|| graph.add_node(c1));
        index_2 = *node_to_index
            .entry(c2)
            .or_insert_with(|| graph.add_node(c2));
        graph.add_edge(index_1, index_2, ());
    }
    graph
}

#[aoc(day7, part1)]
pub fn solve_day7_part1(input: &str) -> String {
    let mut graph = generate_day7(input);
    let mut order = String::new();
    while graph.node_count() > 0 {
        let node = graph
            .node_indices()
            .filter(|n| graph.neighbors_directed(*n, Direction::Incoming).count() == 0)
            .min_by(|a, b| graph[*a].cmp(&graph[*b]))
            .unwrap();
        order.push(graph[node]);
        graph.remove_node(node);
    }
    order
}

#[inline]
fn get_time(c: char) -> usize {
    (c as u8 - b'A') as usize + 1 + 60
}

#[aoc(day7, part2)]
pub fn solve_day7_part2(input: &str) -> usize {
    let mut graph = generate_day7(input);
    let (mut time, mut elf_nodes, mut elf_times) = (0, [None; 5], [0; 5]);
    let mut not_assigned: HashSet<_> = graph.node_indices().collect();
    let (mut time_flag, mut zero_indices, mut min_time);
    while graph.node_count() > 0 {
        time_flag = false;
        zero_indices = Vec::new();
        min_time = (*elf_times.iter().min().unwrap()).max(1);
        for i in 0..elf_times.len() {
            if elf_times[i] > 0 {
                elf_times[i] -= min_time;
                time_flag = true;
            }
            if elf_times[i] == 0 {
                zero_indices.push(i);
                if let Some(node) = elf_nodes[i] {
                    graph.remove_node(node);
                    elf_nodes[i] = None
                }
            }
        }
        if time_flag {
            time += min_time;
        }
        if !zero_indices.is_empty() {
            let nodes = not_assigned
                .iter()
                .filter(|n| graph.neighbors_directed(**n, Direction::Incoming).count() == 0)
                .cloned()
                .sorted_by(|a, b| graph[*a].cmp(&graph[*b]));
            for i in 0..(zero_indices.len().min(nodes.len())) {
                elf_times[zero_indices[i]] = get_time(graph[nodes[i]]);
                elf_nodes[zero_indices[i]] = Some(nodes[i]);
                not_assigned.remove(&nodes[i]);
            }
        }
    }
    time
}
