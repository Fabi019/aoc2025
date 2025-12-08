use std::collections::HashSet;

aoc2025::main!("../../assets/day08.txt");

type PreparedInput = (Vec<(i64, i64, i64)>, Vec<(i64, (usize, usize))>);

fn prepare_input(input: &str) -> PreparedInput {
    let positions = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|num| num.parse::<i64>().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    // Calculate the distances between all pairs
    let mut distances = Vec::new();
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let dx = (positions[i].0 - positions[j].0).abs();
            let dy = (positions[i].1 - positions[j].1).abs();
            let dz = (positions[i].2 - positions[j].2).abs();
            distances.push((dx * dx + dy * dy + dz * dz, (i, j)));
        }
    }

    // Sort pairs by distance
    distances.sort_unstable_by_key(|d| d.0);
    (positions, distances)
}

fn part1(input: &str) -> usize {
    let (_, distances) = prepare_input(input);
    let max_connections = if cfg!(test) { 10 } else { 1000 };

    let mut circuits = Vec::new();
    for (_, (i, j)) in distances.into_iter().take(max_connections) {
        process_connection(i, j, &mut circuits);
    }

    // Sort by largest group
    circuits.sort_unstable_by_key(HashSet::len);
    circuits
        .iter()
        .rev()
        .take(3)
        .map(HashSet::len)
        .product::<usize>()
}

fn process_connection(i: usize, j: usize, circuits: &mut Vec<HashSet<usize>>) {
    let group1 = circuits.iter().position(|g| g.contains(&i));
    let group2 = circuits.iter().position(|g| g.contains(&j));

    match (group1, group2) {
        (Some(g1), Some(g2)) if g1 != g2 => {
            let to_merge = circuits.remove(g2);
            let new_index = if g2 > g1 { g1 } else { g1 - 1 };
            circuits[new_index].extend(to_merge);
        }
        (Some(g1), None) => _ = circuits[g1].insert(j),
        (None, Some(g2)) => _ = circuits[g2].insert(i),
        (None, None) => {
            let mut new_group = HashSet::new();
            new_group.insert(i);
            new_group.insert(j);
            circuits.push(new_group);
        }
        _ => {}
    };
}

fn part2(input: &str) -> i64 {
    let (positions, distances) = prepare_input(input);

    let mut circuits = Vec::new();
    let mut last_pair = None;

    for (_, (i, j)) in distances {
        process_connection(i, j, &mut circuits);

        // Check if all points are in one group
        if circuits.len() == 1 && circuits[0].len() == positions.len() {
            last_pair = Some((i, j));
            break;
        }
    }

    let (i, j) = last_pair.unwrap();
    positions[i].0 * positions[j].0
}

aoc2025::test!(
    "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
",
    40,
    25272
);
