#![allow(dead_code)]
use hashbrown::HashMap;
use itertools::Itertools;

#[aoc_generator(day9)]
pub fn generate_day9(input: &str) -> Box<(usize, usize)> {
    let parts: Vec<_> = input.split_whitespace().collect();
    Box::new((parts[0].parse().unwrap(), parts[6].parse().unwrap()))
}

#[allow(dead_code)]
fn get_circular_index(length: usize, offset: isize) -> usize {
    let new_index = offset;
    if new_index >= 0 && new_index < length as isize {
        new_index as usize
    } else if new_index < 0 {
        length - (new_index.abs() as usize)
    } else {
        (new_index as usize) - length
    }
}

//#[aoc(day9, part1)]
pub fn test_day9(_: &(usize, usize)) -> usize {
    println!("{} 32", solve_day9_part1(&(9, 25)));
    println!("{} 8317", solve_day9_part1(&(10, 1618)));
    println!("{} 146373", solve_day9_part1(&(13, 7999)));
    println!("{} 2764", solve_day9_part1(&(17, 1104)));
    println!("{} 54718", solve_day9_part1(&(21, 6111)));
    println!("{} 37305", solve_day9_part1(&(30, 5807)));
    0
}

pub struct GameCircle {
    circle: Vec<usize>,
    num_marbles: usize,
    num_players: usize,
    num_marbles_used: usize,
    current_marble_index: usize,
    current_player: usize,
    scores: HashMap<usize, usize>,
}

impl GameCircle {
    fn new(num_players: usize, num_marbles: usize) -> Self {
        GameCircle {
            circle: vec![0],
            num_marbles,
            num_players,
            num_marbles_used: 1,
            current_marble_index: 0,
            current_player: 1,
            scores: HashMap::new(),
        }
    }

    fn move_player(&mut self, offset: usize) {
        self.current_player += offset;
        if self.current_player > self.num_players {
            self.current_player -= self.num_players;
        }
    }

    fn add_score(&mut self, score: usize) {
        *self.scores.entry(self.current_player).or_insert(0) += score;
    }

    fn get_circular_index(&self, offset: isize) -> usize {
        let new_index = offset;
        if new_index >= 0 && new_index < self.circle.len() as isize {
            new_index as usize
        } else if new_index < 0 {
            self.circle.len() - (new_index.abs() as usize)
        } else {
            (new_index as usize) - self.circle.len()
        }
    }

    fn interleave(&mut self, interleave_index: usize, num_to_add: usize) {
        self.circle = [
            &self.circle[..interleave_index],
            &(interleave_index..self.circle.len())
                .map(|i| self.circle[i])
                .interleave((0..num_to_add).map(|i| i + self.num_marbles_used))
                .collect::<Vec<_>>(),
        ]
        .concat();
        self.num_marbles_used += num_to_add;
        self.current_marble_index = interleave_index + num_to_add * 2 - 1;
        self.move_player(num_to_add + 1);
    }

    fn next_circle(&mut self) {
        let interleave_index = self.get_circular_index(self.current_marble_index as isize + 1);
        let num_to_add =
            (self.circle.len() - interleave_index).min(self.num_marbles - self.num_marbles_used);
        if let Some((index, marble)) = (0..num_to_add)
            .map(|i| (i, self.num_marbles_used + i))
            .find(|(_, m)| *m % 23 == 0)
        {
            self.interleave(interleave_index, index);

            // player scores
            let remove_marble_index =
                self.get_circular_index(self.current_marble_index as isize - 7);
            let remove_marble = self.circle[remove_marble_index];
            self.circle = [
                &self.circle[..remove_marble_index],
                &self.circle[remove_marble_index + 1..],
            ]
            .concat();
            self.add_score(marble + remove_marble);
            self.num_marbles_used += 1;
            self.current_marble_index = self.get_circular_index(remove_marble_index as isize);
            self.move_player(1);
        // continue
        } else {
            self.interleave(interleave_index, num_to_add);
        }
    }
}

fn get_max_score_2(num_players: usize, num_marbles: usize) -> usize {
    let mut game = GameCircle::new(num_players, num_marbles);
    loop {
        //        println!("{:?}", game.circle);
        if game.num_marbles_used >= num_marbles {
            break;
        }
        game.next_circle()
    }
    *game.scores.values().max().unwrap()
}

#[allow(dead_code)]
fn get_max_score(num_players: usize, num_marbles: usize) -> usize {
    let mut circle = vec![0];
    let mut current_marble_index = 0;
    let mut current_player = 0;
    let mut scores = HashMap::new();
    for i in 1..=num_marbles {
        current_player += 1;
        if current_player >= num_players {
            current_player = 0;
        }
        if i % 23 == 0 {
            let remove_marble_index =
                get_circular_index(circle.len(), current_marble_index as isize - 7);
            let remove_marble = circle[remove_marble_index];
            circle = [
                &circle[..remove_marble_index],
                &circle[remove_marble_index + 1..],
            ]
            .concat();
            *scores.entry(current_player).or_insert(0) += i + remove_marble;
            current_marble_index = get_circular_index(circle.len(), remove_marble_index as isize);
        } else {
            let insert_index = get_circular_index(circle.len(), current_marble_index as isize + 1);
            circle = [
                &circle[..=insert_index],
                &[i],
                &circle[(insert_index + 1)..],
            ]
            .concat();
            current_marble_index = get_circular_index(circle.len(), insert_index as isize + 1);
        }
    }
    *scores.values().max().unwrap()
}

#[aoc(day9, part1)]
pub fn solve_day9_part1(input: &(usize, usize)) -> usize {
    let (num_players, num_marbles) = (input.0, input.1);
    get_max_score(num_players, num_marbles)
}

#[aoc(day9, part2)]
pub fn solve_day9_part2(input: &(usize, usize)) -> usize {
    get_max_score(input.0, input.1 * 100)
}
