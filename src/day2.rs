#[aoc(day2, part1)]
pub fn solve_day2_part1(input: &str) -> usize {
    let box_ids: Vec<_> = input.split('\n').collect();
    let alphabet: Vec<_> = (b'a'..b'z').map(|x| x as char).collect();
    let letter_counts: Vec<_> = box_ids
        .into_iter()
        .map(|w| {
            (
                is_letter_count(w, &alphabet, 2),
                is_letter_count(w, &alphabet, 3),
            )
        })
        .collect();
    letter_counts.iter().filter(|(x, _)| *x).count()
        * letter_counts.into_iter().filter(|(_, x)| *x).count()
}

#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &str) -> String {
    let box_ids: Vec<Vec<_>> = input.split('\n').map(|s| s.chars().collect()).collect();
    for i in 0..(box_ids.len() - 1) {
        for j in (i + 1)..box_ids.len() {
            if get_hamming(&box_ids[i], &box_ids[j]) == 1 {
                return box_ids[i]
                    .iter()
                    .zip(box_ids[j].iter())
                    .filter(|(c1, c2)| c1 == c2)
                    .map(|(c, _)| *c)
                    .collect();
            }
        }
    }
    String::new()
}

fn get_hamming(word_1: &[char], word_2: &[char]) -> usize {
    word_1
        .iter()
        .zip(word_2.iter())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn is_letter_count(word: &str, alphabet: &[char], number: usize) -> bool {
    let word_chars: Vec<_> = word.chars().collect();
    for letter in alphabet {
        if word_chars.iter().filter(|c| *c == letter).count() == number {
            return true;
        }
    }
    false
}
