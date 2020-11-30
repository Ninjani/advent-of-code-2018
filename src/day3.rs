use itertools::Itertools;
use ndarray::{Array2, ArrayView};

#[derive(PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Rectangle {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

fn parse_claim(line: &str) -> Rectangle {
    let (id, loc_dims) = line.split('@').collect_tuple().unwrap();
    let id = id.split('#').collect::<Vec<_>>()[1]
        .trim()
        .parse::<usize>()
        .unwrap();
    let (loc, dims) = loc_dims.split(": ").collect_tuple().unwrap();
    let (left, top) = loc
        .split(',')
        .map(|l| l.trim().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let (width, height) = dims
        .split('x')
        .map(|d| d.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    Rectangle {
        id,
        left,
        top,
        width,
        height,
    }
}

#[aoc_generator(day3)]
pub fn generate_day3(input: &str) -> Vec<Rectangle> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|claim| parse_claim(claim))
        .collect()
}

fn get_num_claims_per_square(rectangles: &[Rectangle]) -> Array2<usize> {
    let (mut max_x, mut max_y) = (0, 0);
    let (mut rect_x, mut rect_y);
    for rectangle in rectangles {
        rect_x = rectangle.left + rectangle.width;
        if rect_x > max_x {
            max_x = rect_x;
        }
        rect_y = rectangle.top + rectangle.height;
        if rect_y > max_y {
            max_y = rect_y;
        }
    }
    let mut claims = Array2::zeros((max_x, max_y));
    for rectangle in rectangles {
        let mut slice = claims.slice_mut(s![
            rectangle.left..rectangle.left + rectangle.width,
            rectangle.top..rectangle.top + rectangle.height
        ]);
        slice += &ArrayView::from(&[1]);
    }
    claims
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &[Rectangle]) -> usize {
    get_num_claims_per_square(input)
        .iter()
        .filter(|num_ids| **num_ids >= 2)
        .count()
}

#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &[Rectangle]) -> usize {
    let claims = get_num_claims_per_square(input);
    for rectangle in input {
        if claims.slice(s![
            rectangle.left..rectangle.left + rectangle.width,
            rectangle.top..rectangle.top + rectangle.height
        ]) == Array2::ones((rectangle.width, rectangle.height))
        {
            return rectangle.id;
        }
    }
    0
}
