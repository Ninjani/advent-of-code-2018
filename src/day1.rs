use hashbrown::HashSet;

#[aoc(day1, part1)]
pub fn solve_day1_part1(input: &str) -> isize {
    input
        .split('\n')
        .filter_map(|s| s.parse::<isize>().ok())
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_day1_part2(input: &str) -> isize {
    let input_list: Vec<_> = input
        .split('\n')
        .filter_map(|s| s.parse::<isize>().ok())
        .collect();
    let (mut frequencies, mut current_frequency) = (HashSet::new(), 0);
    frequencies.insert(current_frequency);
    loop {
        for i in &input_list {
            current_frequency += *i;
            if frequencies.contains(&current_frequency) {
                return current_frequency;
            }
            frequencies.insert(current_frequency);
        }
    }
}

#[aoc(day1, part2, iterator)]
pub fn solve_day1_part2_iterator(input: &str) -> isize {
    let input_list: Vec<_> = input
        .split('\n')
        .filter_map(|s| s.parse::<isize>().ok())
        .collect();
    let (mut frequencies, mut current_frequency) = (HashSet::new(), 0);
    frequencies.insert(current_frequency);
    input_list.into_iter().cycle().find(|c| {
        current_frequency += c;
        !frequencies.insert(current_frequency) // insert returns false if key exists
    });
    current_frequency
}
