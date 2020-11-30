use hashbrown::{HashSet, HashMap};
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Group {
    units: usize,
    hit_points: usize,
    weak_to: Vec<String>,
    immune_to: Vec<String>,
    attack_damage: usize,
    attack_type: String,
    initiative: usize,
}

impl Group {
    fn parse(lines: &str) -> Vec<Self> {
        let group_re =
            Regex::new(r"([0-9]+) units each with ([0-9]+) hit points (?:\(([a-z,; ]+)\) )??with an attack that does ([0-9]+) ([a-z]+) damage at initiative ([0-9]+)").unwrap();
        let weak_re = Regex::new(r".*weak to ([a-z, ]+)").unwrap();
        let immune_re = Regex::new(r".*immune to ([a-z, ]+)").unwrap();
        lines
            .split('\n')
            .skip(1)
            .map(|line| {
                let group = group_re.captures(line).unwrap();
                let (mut weak_to, mut immune_to) = (Vec::new(), Vec::new());
                if let Some(part) = group.get(3) {
                    if let Some(weak) = weak_re.captures(&part.as_str()) {
                        weak_to.extend(weak[1].split(", ").map(|s| s.to_owned()));
                    }
                    if let Some(immune) = immune_re.captures(&part.as_str()) {
                        immune_to.extend(immune[1].split(", ").map(|s| s.to_owned()));
                    }
                }
                let (units, attack_damage) = (group[1].parse().unwrap(), group[4].parse().unwrap());
                Group {
                    units,
                    hit_points: group[2].parse().unwrap(),
                    weak_to,
                    immune_to,
                    attack_damage,
                    attack_type: group[5].to_string(),
                    initiative: group[6].parse().unwrap(),
                }
            })
            .collect()
    }

    fn calculate_damage(&self, enemy: &Group) -> usize {
        let multiplier = if enemy.weak_to.contains(&self.attack_type) {
            2
        } else if enemy.immune_to.contains(&self.attack_type) {
            0
        } else {
            1
        };
        multiplier * self.effective_power()
    }

    pub fn choose_target(&self, enemies: &HashMap<usize, &Group>) -> Option<usize> {
        let enemies: Vec<_> = enemies
            .iter()
            .filter(|(_, enemy)| self.calculate_damage(enemy) > 0).collect();
        if enemies.is_empty() {
            None
        } else {
            Some(*enemies
                .into_iter()
                .max_by(|a, b| {
                    (
                        self.calculate_damage(a.1),
                        a.1.effective_power(),
                        a.1.initiative,
                    ).cmp(&(
                        self.calculate_damage(b.1),
                        b.1.effective_power(),
                        b.1.initiative,
                    ))
                })
                .unwrap()
                .0)
        }
    }

    fn take_damage(&mut self, damage: usize) -> bool {
        let mut damage = damage;
        while damage >= self.hit_points {
            self.units -= 1;
            if self.units == 0 {
                return true;
            }
            damage -= self.hit_points;
        }
        false
    }

    pub fn attack(&mut self, attacker: &Group) -> bool {
        let damage = attacker.calculate_damage(&self);
        self.take_damage(damage)
    }

    pub fn effective_power(&self) -> usize {
        self.units * self.attack_damage
    }
}

#[derive(Debug, Clone)]
pub struct Battle {
    immune_system: HashMap<usize, Group>,
    infection: HashMap<usize, Group>,
}

impl Battle {
    pub fn new(immune_system_groups: Vec<Group>, infection_groups: Vec<Group>) -> Self {
        let num_immune_system_groups = immune_system_groups.len();
        Battle {
            immune_system: immune_system_groups
                .into_iter()
                .enumerate()
                .collect(),
            infection: infection_groups.into_iter().enumerate().map(|(i, g)| (i + num_immune_system_groups, g)).collect()
        }
    }

    pub fn target_selection(&self) -> HashMap<usize, usize> {
        let order = self
            .immune_system
            .iter()
            .chain(self.infection.iter())
            .sorted_by(|b, a| {
                (a.1.effective_power(), a.1.initiative).cmp(&(b.1.effective_power(), b.1.initiative))
            });
        let mut chosen = HashSet::new();
        order
            .into_iter()
            .filter_map(|(i, group)| {
                let enemies = if self.immune_system.contains_key(i) {
                    self.infection.iter().filter(|(i, _)| !chosen.contains(*i)).map(|(i, g)| (*i, g)).collect()
                } else {
                    self.immune_system.iter().filter(|(i, _)| !chosen.contains(*i)).map(|(i, g)| (*i, g)).collect()
                };
                if let Some(enemy) = group.choose_target(&enemies) {
                    chosen.insert(enemy);
                    Some((*i, enemy))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn attack(&mut self, targets: &HashMap<usize, usize>) {
        let order = targets.keys()
            .sorted_by(|b, a| {
                let a_group = if self.immune_system.contains_key(a) {self.immune_system[a].initiative} else {self.infection[a].initiative};
                let b_group = if self.immune_system.contains_key(b) {self.immune_system[b].initiative} else {self.infection[b].initiative};
                a_group.cmp(&b_group)
            });
        for unit in order {
            if !self.immune_system.contains_key(unit) && !self.infection.contains_key(unit){
                continue;
            }
            let enemy_index = targets[unit];
            if !self.immune_system.contains_key(&enemy_index) && !self.infection.contains_key(&enemy_index) {
                continue;
            }
            let against_infection = if self.immune_system.contains_key(unit) { true } else { false };
            let mut killed = false;
            if against_infection {
                if let Some(enemy_group) = self.infection.get_mut(&enemy_index) {
                    killed = enemy_group.attack(&self.immune_system[unit])
                }
                if killed {
                    self.infection.remove(&enemy_index);
                }
            } else {
                if let Some(enemy_group) = self.immune_system.get_mut(&enemy_index) {
                    killed = enemy_group.attack(&self.infection[unit]);
                }
                if killed {
                    self.immune_system.remove(&enemy_index);
                }
            }
        }
    }

    pub fn boost_immune_system(&mut self, boost_value: usize) {
        for group in self.immune_system.values_mut() {
            group.attack_damage += boost_value;
        }
    }

    pub fn battle(&mut self) {
        let mut num_fights = 0;
        loop {
            if num_fights > 3000 || self.immune_system.is_empty() || self.infection.is_empty() {
                break;
            }
            let targets = self.target_selection();
            if targets.is_empty() {
                break;
            }
            self.attack(&targets);
            num_fights += 1;
        }
    }
}

#[aoc_generator(day24)]
pub fn generate_day24(input: &str) -> Box<Battle> {
    let (immune_system, infection) = input.split("\n\n").collect_tuple().unwrap();
    let immune_system_groups = Group::parse(immune_system);
    let infection_groups = Group::parse(infection);
    Box::new(Battle::new(immune_system_groups, infection_groups))
}

#[aoc(day24, part1)]
pub fn solve_day24_part1(input: &Battle) -> usize {
    let mut battle = input.clone();
    battle.battle();
    let immune_system_sum = battle.immune_system.values().map(|g| g.units).sum::<usize>();
    let infection_sum = battle.infection.values().map(|g| g.units).sum();
    immune_system_sum.max(infection_sum)
}

#[aoc(day24, part2)]
pub fn solve_day24_part2(input: &Battle) -> usize {
    for boost in 30.. {
        let mut battle = input.clone();
        battle.boost_immune_system(boost);
        battle.battle();
        if battle.infection.is_empty() {
            return battle.immune_system.values().map(|g| g.units).sum()
        }
    }
    0
}
