#[aoc(day14, part1)]
pub fn solve_day14_part1(input: &str) -> String {
    let num_rounds = input.parse::<usize>().unwrap();
    let mut leaderboard = vec![3, 7];
    let (mut first_index, mut second_index) = (0, 1);
    let mut total;
    loop {
        total = leaderboard[first_index] + leaderboard[second_index];
        if total >= 10 {
            leaderboard.push(total / 10);
            leaderboard.push(total % 10);
        } else {
            leaderboard.push(total);
        }
        if leaderboard.len() > num_rounds + 10 {
            return leaderboard[num_rounds..num_rounds + 10]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("");
        }
        first_index += leaderboard[first_index] + 1;
        first_index %= leaderboard.len();
        second_index += leaderboard[second_index] + 1;
        second_index %= leaderboard.len();
    }
}

#[aoc(day14, part2)]
pub fn solve_day14_part2(input: &str) -> usize {
    let puzzle: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut leaderboard = vec![3, 7];
    let (mut first_index, mut second_index) = (0, 1);
    let mut total;
    loop {
        total = leaderboard[first_index] + leaderboard[second_index];
        if total >= 10 {
            leaderboard.push(total / 10);
            leaderboard.push(total % 10);
        } else {
            leaderboard.push(total);
        }
        if leaderboard.len() > puzzle.len() {
            if leaderboard[leaderboard.len() - puzzle.len()..] == puzzle[..] {
                return leaderboard.len() - puzzle.len();
            } else if leaderboard[leaderboard.len() - puzzle.len() - 1..leaderboard.len() - 1]
                == puzzle[..]
            {
                return leaderboard.len() - puzzle.len() - 1;
            }
        }
        first_index += leaderboard[first_index] + 1;
        first_index %= leaderboard.len();
        second_index += leaderboard[second_index] + 1;
        second_index %= leaderboard.len();
    }
}
