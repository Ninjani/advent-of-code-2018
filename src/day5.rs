#[inline]
fn is_opposite_polarity(c1: char, c2: char) -> bool {
    (c1 != c2) && (c1.to_ascii_lowercase() == c2.to_ascii_lowercase())
}

fn react_polymer(input: &str, minus_letter: Option<char>) -> usize {
    let mut output = Vec::new();
    for character in input.trim().chars() {
        if Some(character.to_ascii_lowercase()) == minus_letter {
            // ignore letter
        } else if !output.is_empty() && is_opposite_polarity(character, *output.last().unwrap()) {
            output.pop();
        } else {
            output.push(character);
        }
    }
    output.len()
}

#[allow(dead_code)]
fn react_polymer_slow(input: &str) -> usize {
    let mut input: Vec<_> = input.trim().chars().collect();
    let (mut new_input, mut index, mut flag);
    loop {
        new_input = Vec::new();
        index = 1;
        flag = false;
        while index < input.len() {
            if is_opposite_polarity(input[index - 1], input[index]) {
                flag = true;
                index += 2;
            } else {
                new_input.push(input[index - 1]);
                index += 1;
            }
        }
        if !flag {
            break;
        }
        if index == input.len() {
            new_input.push(input[index - 1]);
        }
        input = new_input;
    }
    input.len()
}

#[aoc(day5, part1)]
pub fn solve_day5_part1(input: &str) -> usize {
    react_polymer(input, None)
}

#[aoc(day5, part2)]
pub fn solve_day5_part2(input: &str) -> usize {
    (b'a'..=b'z')
        .map(|a| react_polymer(input, Some(a as char)))
        .min()
        .unwrap()
}
