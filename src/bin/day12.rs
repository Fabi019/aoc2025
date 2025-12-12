use std::collections::{BTreeSet, HashSet};

aoc2025::main!("../../assets/day12.txt");

type ShapeVariants = Vec<Vec<Vec<(usize, usize)>>>;

fn parse_shapes(split: &[&str]) -> ShapeVariants {
    split[0..split.len() - 1]
        .iter()
        .map(|s| {
            let mut lines = s.lines();
            let _idx = lines.next();
            let mut positions = BTreeSet::new();
            for (y, l) in lines.enumerate() {
                for (x, c) in l.as_bytes().iter().enumerate() {
                    if c == &b'#' {
                        positions.insert((x, y));
                    }
                }
            }
            positions
        })
        .map(|s| {
            let mut permutations: HashSet<BTreeSet<_>> = HashSet::from_iter([
                s.clone(),
                s.iter().map(|&(x, y)| (y, 2 - x)).collect(), // rotate 90
                s.iter().map(|&(x, y)| (2 - x, 2 - y)).collect(), // rotate 180
                s.iter().map(|&(x, y)| (2 - y, x)).collect(), // rotate 270
            ]);

            let flipped = permutations
                .iter()
                .map(|s| s.iter().map(|&(x, y)| (2 - x, y)).collect::<BTreeSet<_>>())
                .collect::<HashSet<_>>();
            permutations.extend(flipped);
            permutations
        })
        .map(|m| Vec::from_iter(m.into_iter().map(Vec::from_iter)))
        .collect::<ShapeVariants>()
}

fn part1(input: &str) -> u32 {
    let split = input.split("\n\n").collect::<Vec<_>>();
    let shapes = parse_shapes(&split);
    let spaces = split[split.len() - 1].lines().map(|l| {
        let (size, shapes) = l.split_once(": ").unwrap();
        let (width, height) = size.split_once("x").unwrap();
        let shapes = shapes.split_whitespace().map(|s| s.parse::<u32>().unwrap());
        (
            (
                width.parse::<usize>().unwrap(),
                height.parse::<usize>().unwrap(),
            ),
            shapes.collect::<Vec<_>>(),
        )
    });

    for (i, variant) in shapes.iter().enumerate() {
        println!("Shape: {i}");
        for s in variant.iter() {
            for x in 0..3 {
                for y in 0..3 {
                    if s.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(".")
                    }
                }
                println!();
            }
            println!();
        }
    }

    let mut possible = 0;

    for ((w, h), s) in spaces {
        let required = s
            .iter()
            .enumerate()
            .map(|(i, &n)| shapes[i][0].len() * n as usize)
            .sum::<usize>();

        println!("{w}x{h}={}: {:?} = {}", w * h, s, required);

        if required > w * h {
            println!("Impossible pattern");
            continue;
        }

        let max_vert = w / 3;
        let max_horiz = h / 3;

        if (s.iter().sum::<u32>() as usize) < max_horiz * max_vert {
            possible += 1;
            continue;
        }

        let board = vec![vec![false; w]; h];
        if solve_recursive(&board, &shapes, &vec![0; s.len()], &s) {
            println!("is solvable!");
            possible += 1;
        }
    }

    possible
}

fn solve_recursive(
    board: &[Vec<bool>],
    shapes: &ShapeVariants,
    counts: &[u32],
    target_counts: &[u32],
) -> bool {
    if counts == target_counts {
        return true;
    }

    for (i, shape) in shapes.iter().enumerate() {
        if counts[i] == target_counts[i] {
            continue;
        }

        for variant in shape {
            for y in 0..board.len() - 2 {
                for x in 0..board[0].len() - 2 {
                    if let Some(new_board) = place_at_pos(board, variant, (x, y)) {
                        let mut new_counts = counts.to_owned();
                        new_counts[i] += 1;
                        if solve_recursive(&new_board, shapes, &new_counts, target_counts) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

fn place_at_pos(
    board: &[Vec<bool>],
    shape: &[(usize, usize)],
    (ox, oy): (usize, usize),
) -> Option<Vec<Vec<bool>>> {
    for &(x, y) in shape {
        let (x, y) = (x + ox, y + oy);
        if y > board.len() || x > board[y].len() || board[y][x] {
            return None;
        }
    }
    let mut b = board.to_owned();
    for &(x, y) in shape {
        b[y + oy][x + ox] = true;
    }
    Some(b)
}

fn part2(_input: &str) -> u32 {
    0
}

aoc2025::test!(
    "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
",
    2,
    0
);
