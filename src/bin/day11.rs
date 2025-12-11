use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

aoc2025::main!("../../assets/day11.txt");

fn part1(input: &str) -> u32 {
    let graph = input
        .lines()
        .map(|l| {
            let (key, value) = l.split_once(": ").unwrap();
            let value = value.split_ascii_whitespace().collect::<Vec<_>>();
            (key, value)
        })
        .collect::<HashMap<_, _>>();

    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.extend(&graph["you"]);

    let mut paths = 0;

    while let Some(current) = queue.pop_front() {
        if current == "out" {
            paths += 1;
            continue;
        }

        queue.extend(&graph[current]);
    }

    paths
}

fn part2(input: &str) -> usize {
    let graph = input
        .lines()
        .map(|l| {
            let (key, value) = l.split_once(": ").unwrap();
            let value = value.split_ascii_whitespace().collect::<Vec<_>>();
            (key, value)
        })
        .collect::<HashMap<_, _>>();

    CACHE.with(|cache| cache.borrow_mut().clear());
    search_recursive(&graph, "svr", "out", false, false)
}

thread_local! {
    static CACHE: RefCell<HashMap<(String, bool, bool), usize>> = RefCell::new(HashMap::new());
}

fn search_recursive(
    graph: &HashMap<&str, Vec<&str>>,
    current: &str,
    end: &str,
    mut fft: bool,
    mut dac: bool,
) -> usize {
    if current == end {
        return if fft && dac { 1 } else { 0 };
    }

    fft |= current == "fft";
    dac |= current == "dac";

    if let Some(e) = CACHE.with(|cache| {
        cache
            .borrow()
            .get(&(current.to_string(), fft, dac))
            .cloned()
    }) {
        return e;
    }

    let sum = graph[current]
        .iter()
        .map(|c| search_recursive(graph, c, end, fft, dac))
        .sum::<usize>();

    CACHE.with(|cache| {
        cache
            .borrow_mut()
            .insert((current.to_string(), fft, dac), sum);
    });

    sum
}

aoc2025::test!(
    "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
",
    5,
    "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
",
    2
);
