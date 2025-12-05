aoc2025::main!("../../assets/day05.txt");

fn part1(input: &str) -> usize {
    let (ranges, nums) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    nums.lines()
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|&n| {
            for &(start, end) in &ranges {
                if n >= start && n <= end {
                    return true;
                }
            }
            false
        })
        .count()
}

fn part2(input: &str) -> u64 {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    ranges.sort_unstable_by_key(|&(start, _)| start);

    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
    for r @ (start, end) in ranges {
        // When the start of the current range falls into the previous range
        // extend previous range to also include current range
        if let Some(prev) = merged.last_mut()
            && start <= prev.1 + 1
        {
            prev.1 = prev.1.max(end);
            continue;
        }
        merged.push(r);
    }

    merged
        .iter()
        .fold(0, |acc, (start, end)| acc + end - start + 1)
}

aoc2025::test!(
    "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    3,
    14
);
