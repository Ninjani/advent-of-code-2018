use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Plants = (Vec<u32>, HashMap<Vec<u32>, u32>);

#[aoc_generator(day12)]
pub fn generate_day12(input: &str) -> Box<Plants> {
    let lines: Vec<_> = input.split('\n').filter(|l| !l.is_empty()).collect();
    let (_, initial_state_chars) = lines[0].split(": ").collect_tuple().unwrap();
    let initial_state = initial_state_chars
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    let rules = lines[1..]
        .iter()
        .map(|line| {
            let (rule, outcome) = line.split(" => ").collect_tuple().unwrap();
            (
                rule.chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .collect::<Vec<u32>>(),
                if outcome == "#" { 1 } else { 0 },
            )
        })
        .collect();
    Box::new((initial_state, rules))
}

fn get_next_generation(state: &[u32], rules: &HashMap<Vec<u32>, u32>) -> (Vec<u32>, i64) {
    let (mut num_add_front, mut num_add_back) = (0, 0);
    for i in (1..5).rev() {
        let part = [&(0..i).map(|_| 0).collect::<Vec<_>>(), &state[..5 - i]].concat();
        if let Some(1) = rules.get(&part) {
            num_add_front = i as i64;
            break;
        }
    }
    for i in (1..5).rev() {
        let part = [
            &state[state.len() - 5 + i..],
            &(0..i).map(|_| 0).collect::<Vec<_>>(),
        ]
        .concat();
        if let Some(1) = rules.get(&part) {
            num_add_back = i;
            break;
        }
    }
    let state = [
        &(0..num_add_front).map(|_| 0).collect::<Vec<_>>(),
        state,
        &(0..num_add_back).map(|_| 0).collect::<Vec<_>>(),
    ]
    .concat();
    let mut new_state = state.clone();
    for i in 2..state.len() - 2 {
        if let Some(outcome) = rules.get(&state[i - 2..=i + 2]) {
            new_state[i] = *outcome;
        } else {
            new_state[i] = 0;
        }
    }
    (new_state, num_add_front)
}

fn grow(initial_state: &[u32], rules: &HashMap<Vec<u32>, u32>, num_generations: usize) -> i64 {
    let (mut state, mut start_index) = (initial_state.to_vec(), 0);
    let (mut generation, mut previous_sum) = (None, 0);
    let mut differences = Vec::new();
    for g in 0..num_generations {
        let state_start_index = get_next_generation(&state, &rules);
        state = state_start_index.0;
        start_index += state_start_index.1;
        let new_sum = state
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == 1)
            .map(|(i, _)| i as i64 - start_index)
            .sum::<i64>();
        differences.push(new_sum - previous_sum);
        previous_sum = new_sum;
        if differences.len() > 10 {
            differences = differences[differences.len() - 10..].to_vec();
            if differences.iter().collect::<HashSet<_>>().len() == 1 {
                generation = Some(g);
                break;
            }
        }
    }
    if let Some(g) = generation {
        (num_generations - g - 1) as i64 * differences[0] + previous_sum
    } else {
        previous_sum
    }
}

#[aoc(day12, part1)]
pub fn solve_day12_part1(input: &Plants) -> i64 {
    grow(&input.0, &input.1, 20)
}

#[aoc(day12, part2)]
pub fn solve_day12_part2(input: &Plants) -> i64 {
    grow(&input.0, &input.1, 50_000_000_000)
}
