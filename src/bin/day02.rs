aoc2025::main!("../../assets/day02.txt");

fn part1(input: &str) -> u64 {
    let ranges = input.lines().next().unwrap().split(",").map(|r| {
        let (start, end) = r.split_once("-").unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    ranges.fold(0, |mut acc, (start, end)| {
        'nextNumer: for num in start..=end {
            let s = num.to_string();
            if s.len() % 2 != 0 || s[0..s.len() / 2] != s[s.len() / 2..] {
                continue 'nextNumer;
            }
            acc += num;
        }
        acc
    })
}

fn part2(input: &str) -> u64 {
    let ranges = input.lines().next().unwrap().split(",").map(|r| {
        let (start, end) = r.split_once("-").unwrap();
        (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
    });

    ranges.fold(0, |mut acc, (start, end)| {
        'nextNumer: for num in start..=end {
            let s = num.to_string();
            'nextPattern: for pat_len in 1..=s.len() / 2 {
                let pattern = &s[0..pat_len];
                if s.len() % pat_len != 0 {
                    continue 'nextPattern;
                }
                for i in (pat_len..s.len()).step_by(pat_len) {
                    if &s[i..i + pat_len] != pattern {
                        continue 'nextPattern;
                    }
                }
                acc += num;
                continue 'nextNumer;
            }
        }
        acc
    })
}

aoc2025::test!(
    "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
",
    1227775554,
    4174379265
);
