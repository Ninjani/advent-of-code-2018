use itertools::Itertools;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[aoc_generator(day8)]
pub fn generate_day8(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn build_tree(
    tree: &mut Graph<Vec<usize>, ()>,
    index: usize,
    numbers: &[usize],
) -> (NodeIndex<u32>, usize) {
    let (num_children, num_metadata) = (numbers[index], numbers[index + 1]);
    let parent = tree.add_node(Vec::with_capacity(num_metadata));
    let mut index = index + 2;
    for _ in 0..num_children {
        let (child, new_index) = build_tree(tree, index, &numbers);
        index = new_index;
        tree.add_edge(parent, child, ());
    }
    for _ in 0..num_metadata {
        tree[parent].push(numbers[index]);
        index += 1;
    }
    (parent, index)
}

#[aoc(day8, part1)]
pub fn solve_day8_part1(input: &[usize]) -> usize {
    let mut tree = Graph::new();
    build_tree(&mut tree, 0, input);
    tree.node_indices()
        .map(|n| tree[n].iter().sum::<usize>())
        .sum()
}

fn get_node_value(tree: &Graph<Vec<usize>, ()>, index: NodeIndex<u32>) -> usize {
    if tree.neighbors(index).count() == 0 {
        tree[index].iter().sum()
    } else {
        let children = tree.neighbors(index).sorted();
        tree[index]
            .iter()
            .map(|m| {
                if *m == 0 || *m > children.len() {
                    0
                } else {
                    get_node_value(tree, children[*m - 1])
                }
            })
            .sum()
    }
}

#[aoc(day8, part2)]
pub fn solve_day8_part2(input: &[usize]) -> usize {
    let mut tree = Graph::new();
    let (root, _) = build_tree(&mut tree, 0, input);
    get_node_value(&tree, root)
}
