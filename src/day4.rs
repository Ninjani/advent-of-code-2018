use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use hashbrown::HashMap;
use itertools::Itertools;
use ndarray::{Array2, Axis};

pub struct Entry {
    guard_id: Option<usize>,
    date: DateTime<Utc>,
    minute: usize,
    is_asleep: bool,
}

fn parse_line(line: &str) -> Entry {
    let (date_time, entry) = line.split("] ").collect_tuple().unwrap();
    let mut date = Utc
        .datetime_from_str(&date_time[1..], "%Y-%m-%d %H:%M")
        .unwrap();
    if date.hour() > 0 {
        date = (date + Duration::days(1)).date().and_hms(0, 0, 0);
    }
    let guard_id = if entry.contains('#') {
        Some(
            entry
                .split(' ')
                .filter(|x| x.contains('#'))
                .map(|x| x[1..].parse::<usize>().unwrap())
                .next()
                .unwrap(),
        )
    } else {
        None
    };
    Entry {
        guard_id,
        minute: date.minute() as usize,
        date,
        is_asleep: entry.contains("asleep"),
    }
}

#[aoc_generator(day4)]
pub fn generate_day4(input: &str) -> Vec<Entry> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .sorted_by(|a, b| (a.date).cmp(&(b.date)))
}

fn get_matrix(entries: &[Entry]) -> (HashMap<usize, Vec<usize>>, Array2<usize>) {
    let mut guard_to_date = HashMap::new();
    let (min_date, max_date) = (entries[0].date, entries[entries.len() - 1].date);
    let mut times = Array2::zeros(((max_date - min_date).num_days() as usize + 1, 60));
    for i in 0..entries.len() {
        let days = (entries[i].date - min_date).num_days() as usize;
        if let Some(guard_id) = entries[i].guard_id {
            guard_to_date
                .entry(guard_id)
                .or_insert_with(Vec::new)
                .push(days);
        } else if i < entries.len() - 1 && entries[i].is_asleep && !entries[i + 1].is_asleep {
            times
                .slice_mut(s![days, entries[i].minute..entries[i + 1].minute])
                .fill(1);
        }
    }
    (guard_to_date, times)
}

fn get_max_value_index(array: &Array2<usize>) -> (usize, usize) {
    (0..array.shape()[1])
        .map(|i| (array.column(i).scalar_sum(), i))
        .max()
        .unwrap()
}

#[aoc(day4, part1)]
pub fn solve_day4_part1(input: &[Entry]) -> usize {
    let (guard_to_date, times) = get_matrix(input);
    let (_, max_guard) = guard_to_date
        .iter()
        .map(|(g_id, dates)| (times.select(Axis(0), &dates).scalar_sum(), g_id))
        .max()
        .unwrap();
    let (_max_count, max_minute) =
        get_max_value_index(&times.select(Axis(0), &guard_to_date[&max_guard]));
    max_minute * max_guard
}

#[aoc(day4, part2)]
pub fn solve_day4_part2(input: &[Entry]) -> usize {
    let (guard_to_date, times) = get_matrix(input);
    let ((_max_count, max_minute), max_guard) = guard_to_date
        .iter()
        .map(|(g_id, indices)| (get_max_value_index(&times.select(Axis(0), indices)), g_id))
        .max()
        .unwrap();
    max_minute * max_guard
}
