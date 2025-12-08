use std::collections::HashSet;

aoc2025::main!("../../assets/day08.txt");

fn part1(input: &str) -> u32 {
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

    let mut distances = Vec::new();

    // Find the distances between all pairs
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let dx = (positions[i].0 - positions[j].0).abs();
            let dy = (positions[i].1 - positions[j].1).abs();
            let dz = (positions[i].2 - positions[j].2).abs();
            distances.push(((i, j), f64::sqrt((dx * dx + dy * dy + dz * dz) as f64)));
        }
    }

    // Sort pairs by distance
    distances.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));

    let mut circuits: Vec<HashSet<_>> = Vec::new();

    let max_connections = if cfg!(test) { 10 } else { 1000 };
    let mut connections = 0;
    for ((i, j), d) in &distances {
        println!(
            "Connecting points {i}/{:?} to {j}{:?} distance: {d}",
            positions[*i], positions[*j]
        );

        let group1 = circuits.iter().position(|g| g.contains(i));
        let group2 = circuits.iter().position(|g| g.contains(j));

        match (group1, group2) {
            (Some(g1), Some(g2)) if g1 != g2 => {
                let to_merge = circuits.remove(g2);
                let new_index = if g2 > g1 { g1 } else { g1 - 1 };
                circuits[new_index].extend(to_merge);
            }
            (Some(g1), None) => {
                circuits[g1].insert(*j);
            }
            (None, Some(g2)) => {
                circuits[g2].insert(*i);
            }
            (None, None) => {
                let mut new_group = HashSet::new();
                new_group.insert(*i);
                new_group.insert(*j);
                circuits.push(new_group);
            }
            _ => {}
        }

        connections += 1;
        if connections >= max_connections {
            break;
        }
    }

    // Sort by largest group
    circuits.sort_unstable_by_key(|g| g.len());

    // print connected groups
    for group in &circuits {
        println!("Connected group: {:?}", group);
    }

    circuits
        .iter()
        .rev()
        .take(3)
        .fold(1, |acc, g| acc * g.len() as u32)
}

fn part2(input: &str) -> i64 {
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

    let mut distances = Vec::new();

    // Find the distances between all pairs
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let dx = (positions[i].0 - positions[j].0).abs();
            let dy = (positions[i].1 - positions[j].1).abs();
            let dz = (positions[i].2 - positions[j].2).abs();
            distances.push(((i, j), f64::sqrt((dx * dx + dy * dy + dz * dz) as f64)));
        }
    }

    // Sort pairs by distance
    distances.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));

    let mut circuits: Vec<HashSet<_>> = Vec::new();
    let mut last_pair = None;

    for ((i, j), _) in &distances {
        let group1 = circuits.iter().position(|g| g.contains(i));
        let group2 = circuits.iter().position(|g| g.contains(j));

        match (group1, group2) {
            (Some(g1), Some(g2)) if g1 != g2 => {
                let to_merge = circuits.remove(g2);
                let new_index = if g2 > g1 { g1 } else { g1 - 1 };
                circuits[new_index].extend(to_merge);
            }
            (Some(g1), None) => {
                circuits[g1].insert(*j);
            }
            (None, Some(g2)) => {
                circuits[g2].insert(*i);
            }
            (None, None) => {
                let mut new_group = HashSet::new();
                new_group.insert(*i);
                new_group.insert(*j);
                circuits.push(new_group);
            }
            _ => {}
        }

        // Check if all points are in one group
        if circuits.len() == 1 && circuits[0].len() == positions.len() {
            last_pair = Some((i, j));
            break;
        }
    }

    let (i, j) = last_pair.unwrap();
    positions[*i].0 * positions[*j].0
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
