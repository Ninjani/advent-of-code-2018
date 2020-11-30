use ndarray::Array2;

#[aoc_generator(day11)]
fn generate_day11(input: &str) -> Box<Array2<i32>> {
    let serial_number = input.parse::<usize>().unwrap();
    let mut grid = Array2::zeros((300, 300));
    for i in 0..300 {
        for j in 0..300 {
            grid[(i, j)] = get_battery(i + 1, j + 1, serial_number);
        }
    }
    Box::new(grid)
}

fn get_battery(x: usize, y: usize, serial_number: usize) -> i32 {
    let rack_id = x + 10;
    (((((rack_id * y) + serial_number) * rack_id) / 100) % 10) as i32 - 5
}

#[aoc(day11, part1)]
pub fn solve_day11_part1(grid: &Array2<i32>) -> String {
    let mut grid_battery;
    let mut max_battery = 0;
    let mut best_xy = (0, 0);
    for i in 0..300 - 3 {
        for j in 0..300 - 3 {
            grid_battery = grid.slice(s![i..i + 3, j..j + 3]).sum();
            if grid_battery > max_battery {
                max_battery = grid_battery;
                best_xy = (i, j);
            }
        }
    }
    format!("{},{}", best_xy.0 + 1, best_xy.1 + 1)
}

#[aoc(day11, part2)]
pub fn solve_day11_part2(grid: &Array2<i32>) -> String {
    let mut grid_battery;
    let mut max_battery = 0;
    let mut best_xys = (0, 0, 0);
    for s in 0..300 {
        for i in 0..300 - s {
            for j in 0..300 - s {
                grid_battery = grid.slice(s![i..i + s, j..j + s]).sum();
                if grid_battery > max_battery {
                    max_battery = grid_battery;
                    best_xys = (i, j, s);
                }
            }
        }
    }
    format!("{},{},{}", best_xys.0 + 1, best_xys.1 + 1, best_xys.2)
}
