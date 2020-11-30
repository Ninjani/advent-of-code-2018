use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

pub struct CartSystem {
    num_rows: usize,
    num_cols: usize,
    tracks: HashMap<(usize, usize), Track>,
    carts: Vec<(usize, usize, Direction, u8)>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Track {
    Empty,
    Horizontal,
    Vertical,
    CurveForward,
    CurveBackward,
    Intersection,
}

impl FromStr for Track {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Track, Self::Err> {
        match s.as_bytes().get(0) {
            None => panic!("Empty string"),
            Some(&b' ') => Ok(Track::Empty),
            Some(&b'-') | Some(&b'>') | Some(&b'<') => Ok(Track::Horizontal),
            Some(&b'|') | Some(&b'^') | Some(&b'v') => Ok(Track::Vertical),
            Some(&b'/') => Ok(Track::CurveForward),
            Some(&b'\\') => Ok(Track::CurveBackward),
            Some(&b'+') => Ok(Track::Intersection),
            _ => panic!("Weird track: {}", s),
        }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Track::Empty => write!(f, " "),
            Track::Horizontal => write!(f, "-"),
            Track::Vertical => write!(f, "|"),
            Track::CurveBackward => write!(f, "\\"),
            Track::CurveForward => write!(f, "/"),
            Track::Intersection => write!(f, "+"),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "<"),
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Right => write!(f, ">"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Down,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn(&self, turn: u8) -> Direction {
        match turn {
            1 => self.turn_left(),
            2 => *self,
            3 => self.turn_right(),
            _ => panic!("Weird turn"),
        }
    }
}

impl FromStr for Direction {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Direction, Self::Err> {
        match s.as_bytes().get(0) {
            None => panic!("Empty string"),
            Some(&b'<') => Ok(Direction::Left),
            Some(&b'^') => Ok(Direction::Up),
            Some(&b'v') => Ok(Direction::Down),
            Some(&b'>') => Ok(Direction::Right),
            _ => panic!("Weird direction: {}", s),
        }
    }
}

impl CartSystem {
    fn new(input: &str) -> CartSystem {
        let (num_rows, num_cols) = (
            input.split('\n').count(),
            input.split('\n').map(|l| l.len()).max().unwrap(),
        );
        let mut tracks = HashMap::with_capacity(num_rows * num_cols);
        let mut carts = Vec::new();
        for (i, row) in input.split('\n').enumerate() {
            let mut new_row = if i == 0 {
                (0..26).map(|_| ' ').collect::<String>()
            } else {
                String::new()
            };
            new_row.push_str(row);
            for (j, c) in new_row.chars().enumerate() {
                tracks.insert((i, j), c.to_string().parse().unwrap());
                if "<>^v".contains(c) {
                    carts.push((i, j, c.to_string().parse().unwrap(), 1));
                }
            }
        }
        let mut cart_system = CartSystem {
            num_rows,
            num_cols,
            tracks,
            carts,
        };
        cart_system.smooth();
        cart_system
    }

    fn smooth(&mut self) {
        for (row, column, direction, _) in self.carts.iter() {
            let (row, column) = (*row, *column);
            match *direction {
                Direction::Left => {
                    if let Some(Track::Vertical) = self.tracks.get(&(row + 1, column)) {
                        *self.tracks.entry((row, column)).or_insert(Track::Empty) =
                            Track::CurveBackward;
                    }
                    if let Some(Track::Vertical) = self.tracks.get(&(row - 1, column)) {
                        *self.tracks.entry((row, column)).or_insert(Track::Empty) =
                            Track::CurveForward;
                    }
                }
                Direction::Up => {
                    if let Some(Track::Horizontal) = self.tracks.get(&(row, column + 1)) {
                        *self.tracks.entry((row, column)).or_insert(Track::Empty) =
                            Track::CurveBackward;
                    }
                    if let Some(Track::Horizontal) = self.tracks.get(&(row, column - 1)) {
                        *self.tracks.entry((row, column)).or_insert(Track::Empty) =
                            Track::CurveForward;
                    }
                }
                Direction::Down => {
                    if let Some(Track::Horizontal) = self.tracks.get(&(row, column + 1)) {
                        *self.tracks.entry((row, column)).or_insert(Track::Empty) =
                            Track::CurveForward;
                    }
                    if let Some(Track::Horizontal) = self.tracks.get(&(row, column - 1)) {
                        *self.tracks.entry((row, column)).or_insert(Track::Empty) =
                            Track::CurveBackward;
                    }
                }
                Direction::Right => {
                    if let Some(Track::Vertical) = self.tracks.get(&(row + 1, column)) {
                        *self.tracks.entry((row, column)).or_insert(Track::Empty) =
                            Track::CurveForward;
                    }
                    if let Some(Track::Vertical) = self.tracks.get(&(row - 1, column)) {
                        *self.tracks.entry((row, column)).or_insert(Track::Empty) =
                            Track::CurveBackward;
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    fn display(&self, show_tracks: bool, show_carts: bool) {
        let mut track_string = (0..self.num_rows)
            .map(|r| {
                (0..self.num_cols)
                    .map(|c| {
                        if show_tracks {
                            format!("{}", self.tracks[&(r, c)]).chars().next().unwrap()
                        } else {
                            ' '
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        if show_carts {
            for (row, col, dir, _) in &self.carts {
                track_string[*row][*col] = format!("{}", dir).chars().next().unwrap();
            }
        }
        println!(
            "{}",
            track_string
                .into_iter()
                .map(|v| v.into_iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    fn move_cart(&mut self, index: usize) {
        let (row, column, direction, turn) = self.carts[index];
        let mut turned = false;
        let (new_row, new_column, new_direction) = match direction {
            Direction::Left => match self.tracks.get(&(row, column - 1)) {
                Some(Track::Horizontal) => (row, column - 1, Direction::Left),
                Some(Track::CurveForward) => (row, column - 1, Direction::Down),
                Some(Track::CurveBackward) => (row, column - 1, Direction::Up),
                Some(Track::Intersection) => {
                    turned = true;
                    (row, column - 1, direction.turn(turn))
                }
                _ => panic!("Weird cart, -|"),
            },
            Direction::Up => match self.tracks.get(&(row - 1, column)) {
                Some(Track::Vertical) => (row - 1, column, Direction::Up),
                Some(Track::CurveForward) => (row - 1, column, Direction::Right),
                Some(Track::CurveBackward) => (row - 1, column, Direction::Left),
                Some(Track::Intersection) => {
                    turned = true;
                    (row - 1, column, direction.turn(turn))
                }
                _ => panic!("Weird cart, |-"),
            },
            Direction::Down => match self.tracks.get(&(row + 1, column)) {
                Some(Track::Vertical) => (row + 1, column, Direction::Down),
                Some(Track::CurveForward) => (row + 1, column, Direction::Left),
                Some(Track::CurveBackward) => (row + 1, column, Direction::Right),
                Some(Track::Intersection) => {
                    turned = true;
                    (row + 1, column, direction.turn(turn))
                }
                _ => panic!("Weird cart, |-"),
            },
            Direction::Right => match self.tracks.get(&(row, column + 1)) {
                Some(Track::Horizontal) => (row, column + 1, Direction::Right),
                Some(Track::CurveForward) => (row, column + 1, Direction::Up),
                Some(Track::CurveBackward) => (row, column + 1, Direction::Down),
                Some(Track::Intersection) => {
                    turned = true;
                    (row, column + 1, direction.turn(turn))
                }
                _ => panic!("Weird cart, -|"),
            },
        };
        let mut new_turn = turn;
        if turned {
            new_turn += 1;
            if new_turn > 3 {
                new_turn = 1;
            }
        }
        self.carts[index] = (new_row, new_column, new_direction, new_turn);
    }

    fn tick(&mut self) -> Option<(usize, usize)> {
        let mut positions: HashSet<_> = self.carts.iter().map(|x| (x.0, x.1)).collect();
        let order: Vec<_> = self
            .carts
            .iter()
            .enumerate()
            .sorted_by(|a, b| ((a.1).0, (a.1).1).cmp(&((b.1).0, (b.1).1)))
            .into_iter()
            .map(|(i, _)| i)
            .collect();
        for i in order {
            let (old_row, old_column, _, _) = self.carts[i];
            positions.remove(&(old_row, old_column));
            self.move_cart(i);
            let (row, column, _, _) = self.carts[i];
            if positions.contains(&(row, column)) {
                return Some((row, column));
            }
            positions.insert((row, column));
        }
        None
    }

    fn tick_remove(&mut self) -> Option<(usize, usize)> {
        if self.carts.len() == 1 {
            return Some((self.carts[0].0, self.carts[0].1));
        }
        let mut positions = HashMap::new();
        for i in 0..self.carts.len() {
            positions.insert((self.carts[i].0, self.carts[i].1), i);
        }
        let order: Vec<_> = self
            .carts
            .iter()
            .enumerate()
            .sorted_by(|a, b| ((a.1).0, (a.1).1).cmp(&((b.1).0, (b.1).1)))
            .into_iter()
            .map(|(i, _)| i)
            .collect();
        let mut remove_indices = HashSet::new();
        for i in order {
            let (old_row, old_column, _, _) = self.carts[i];
            positions.remove(&(old_row, old_column));
            self.move_cart(i);
            let (row, column, _, _) = self.carts[i];
            if positions.contains_key(&(row, column)) {
                remove_indices.insert(i);
                remove_indices.insert(positions.remove(&(row, column)).unwrap());
            } else {
                positions.insert((row, column), i);
            }
        }
        self.carts = (0..self.carts.len())
            .filter(|i| !remove_indices.contains(i))
            .map(|i| self.carts[i])
            .collect();
        None
    }
}

#[aoc(day13, part1)]
pub fn solve_day13_part1(input: &str) -> String {
    let mut cart_system = CartSystem::new(input);
    loop {
        if let Some((row, column)) = cart_system.tick() {
            return format!("{},{}", column, row);
        }
    }
}

#[aoc(day13, part2)]
pub fn solve_day13_part2(input: &str) -> String {
    let mut cart_system = CartSystem::new(input);
    loop {
        if let Some((row, column)) = cart_system.tick_remove() {
            return format!("{},{}", column, row);
        }
    }
}
