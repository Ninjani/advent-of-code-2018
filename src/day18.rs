use hashbrown::HashSet;

pub const NUM_ACRES: usize = 50;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Acre {
    Open,
    Trees,
    Lumberyard,
}

impl Acre {
    pub fn change(&self, adjacent: &[Acre]) -> Acre {
        match self {
            Acre::Open => {
                if adjacent.into_iter().filter(|a| **a == Acre::Trees).count() >= 3 {
                    Acre::Trees
                } else {
                    Acre::Open
                }
            }
            Acre::Trees => {
                if adjacent
                    .into_iter()
                    .filter(|a| **a == Acre::Lumberyard)
                    .count()
                    >= 3
                {
                    Acre::Lumberyard
                } else {
                    Acre::Trees
                }
            }
            Acre::Lumberyard => {
                if adjacent.iter().filter(|a| **a == Acre::Lumberyard).count() >= 1
                    && adjacent.iter().filter(|a| **a == Acre::Trees).count() >= 1
                {
                    Acre::Lumberyard
                } else {
                    Acre::Open
                }
            }
        }
    }
}

pub struct LumberCollectionArea {
    area: [[Acre; NUM_ACRES]; NUM_ACRES],
}

impl LumberCollectionArea {
    pub fn read(input: &str) -> Self {
        let mut area = [[Acre::Open; NUM_ACRES]; NUM_ACRES];
        for (i, line) in input.split('\n').enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => area[i][j] = Acre::Open,
                    '|' => area[i][j] = Acre::Trees,
                    '#' => area[i][j] = Acre::Lumberyard,
                    _ => panic!("Unknown acre"),
                }
            }
        }
        LumberCollectionArea { area }
    }

    pub fn get_adjacent(&self, x: usize, y: usize) -> Vec<Acre> {
        let mut adjacent_acres = Vec::new();
        if x > 0 {
            adjacent_acres.push(self.area[x - 1][y]);
            if y > 0 {
                adjacent_acres.push(self.area[x - 1][y - 1]);
            }
            if y < NUM_ACRES - 1 {
                adjacent_acres.push(self.area[x - 1][y + 1]);
            }
        }
        if x < NUM_ACRES - 1 {
            adjacent_acres.push(self.area[x + 1][y]);
            if y > 0 {
                adjacent_acres.push(self.area[x + 1][y - 1]);
            }
            if y < NUM_ACRES - 1 {
                adjacent_acres.push(self.area[x + 1][y + 1]);
            }
        }
        if y > 0 {
            adjacent_acres.push(self.area[x][y - 1]);
        }
        if y < NUM_ACRES - 1 {
            adjacent_acres.push(self.area[x][y + 1]);
        }

        adjacent_acres
    }

    pub fn change(&mut self) -> bool {
        let mut new_area = [[Acre::Open; NUM_ACRES]; NUM_ACRES];
        let mut changed = false;
        for i in 0..NUM_ACRES {
            for j in 0..NUM_ACRES {
                new_area[i][j] = self.area[i][j].change(&self.get_adjacent(i, j));
                if new_area[i][j] != self.area[i][j] {
                    changed = true;
                }
            }
        }
        self.area = new_area;
        changed
    }

    pub fn display(&self) {
        for i in 0..NUM_ACRES {
            for j in 0..NUM_ACRES {
                print!(
                    "{}",
                    match self.area[i][j] {
                        Acre::Open => '.',
                        Acre::Trees => '|',
                        Acre::Lumberyard => '#',
                    }
                )
            }
            println!();
        }
        println!();
    }

    pub fn get_counts(&self) -> (usize, usize) {
        let (mut num_trees, mut num_lumberyards) = (0, 0);
        for i in 0..NUM_ACRES {
            for j in 0..NUM_ACRES {
                match self.area[i][j] {
                    Acre::Trees => num_trees += 1,
                    Acre::Lumberyard => num_lumberyards += 1,
                    _ => (),
                }
            }
        }
        (num_trees, num_lumberyards)
    }
}

#[aoc(day18, part1)]
pub fn solve_day18_part1(input: &str) -> usize {
    let mut area = LumberCollectionArea::read(input);
    for _ in 0..10 {
        area.change();
    }
    let (num_trees, num_lumberyards) = area.get_counts();
    num_trees * num_lumberyards
}

#[aoc(day18, part2)]
pub fn solve_day18_part2(input: &str) -> usize {
    let mut area = LumberCollectionArea::read(input);
    let mut counts = HashSet::new();
    let mut c_t_l;
    let mut num_done = 0;
    let mut found_x = 0;
    loop {
        c_t_l = area.get_counts();
        if found_x == 0 && counts.contains(&c_t_l) {
            found_x += 1;
        }
        if found_x > 0 {
            if !counts.contains(&c_t_l) {
                found_x = 0;
            } else {
                found_x += 1;
                if found_x > 4 {
                    break;
                }
            }
        }
        counts.insert(c_t_l);
        area.change();
        num_done += 1;
    }
    let mut period = 1;
    let mut new_c_t_l;
    loop {
        area.change();
        new_c_t_l = area.get_counts();
        if new_c_t_l == c_t_l {
            break;
        }
        period += 1;
    }
    let num_steps = (1000000000usize - num_done - period) % period;
    for _ in 0..num_steps {
        area.change();
    }
    let (num_trees, num_lumberyards) = area.get_counts();
    num_trees * num_lumberyards
}
