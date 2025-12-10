use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};

aoc2025::main!("../../assets/day10.txt");

fn part1(input: &str) -> usize {
    let machine = input.lines().map(|l| {
        let e = l.split_whitespace().collect::<Vec<_>>();
        let targets = e[0]
            .trim_matches(|c| c == '[' || c == ']')
            .as_bytes()
            .iter()
            .map(|b| b == &b'#')
            .collect::<Vec<_>>();
        let buttons = e[1..e.len() - 1]
            .iter()
            .map(|b| {
                let b = b.trim_matches(|c| c == '(' || c == ')');
                b.split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        (targets, buttons)
    });

    let mut total_presses = 0;
    'next: for (targets, buttons) in machine {
        println!("Target: {:?}, Buttons: {:?}", targets, buttons);

        for i in 1..buttons.len() {
            let options = recurse_buttons(&buttons, 0, 0, i);
            for option in options {
                let mut lights = vec![false; targets.len()];
                for target_button in &option {
                    for &b in &buttons[*target_button] {
                        lights[b] = !lights[b];
                    }
                    if lights == targets {
                        println!("Matched with buttons {:?} ({} presses)", option, i);
                        total_presses += i;
                        continue 'next;
                    }
                }
            }
        }

        unreachable!("No combination found for target {:?}", targets);
    }

    total_presses
}

fn recurse_buttons(
    buttons: &[Vec<usize>],
    start: usize,
    current: usize,
    presses: usize,
) -> Vec<Vec<usize>> {
    let mut results = Vec::new();

    if current == presses || start >= buttons.len() {
        return results;
    }

    for i in start..buttons.len() {
        if current + 1 == presses {
            results.push(vec![i]);
        } else {
            let sub_results = recurse_buttons(buttons, i + 1, current + 1, presses);
            for mut sub in sub_results {
                let mut combo = vec![i];
                combo.append(&mut sub);
                results.push(combo);
            }
        }
    }

    results
}

fn part2(input: &str) -> u32 {
    let machine = input.lines().map(|l| {
        let e = l.split_whitespace().collect::<Vec<_>>();
        let buttons = e[1..e.len() - 1]
            .iter()
            .map(|b| {
                let b = b.trim_matches(|c| c == '(' || c == ')');
                b.split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let targets = e[e.len() - 1]
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        (targets, buttons)
    });

    let mut total = 0;
    for (targets, buttons) in machine {
        let mut vars = variables!();
        let b = (0..buttons.len())
            .map(|i| vars.add(variable().integer().min(0).name(format!("b{}", i))))
            .collect::<Vec<_>>();

        let mut problem = vars
            .minimise(b.iter().sum::<Expression>())
            .using(default_solver);

        for (light_idx, &target) in targets.iter().enumerate() {
            let mut expr = Expression::from(0);
            for (button_idx, button) in buttons.iter().enumerate() {
                if button.contains(&light_idx) {
                    expr = expr + b[button_idx];
                }
            }
            problem = problem.with(expr.eq(target));
        }

        let solution = problem.solve().unwrap();

        for p in b.iter() {
            total += solution.value(*p).round() as u32;
        }
    }

    total
}

aoc2025::test!(
    "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
",
    7,
    33
);
