use std::collections::HashSet;

aoc2025::main!("../../assets/day09.txt");

fn part1(input: &str) -> u64 {
    let corners = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut max_area = 0;
    for i in 0..corners.len() {
        for j in i + 1..corners.len() {
            let (x1, y1) = corners[i];
            let (x2, y2) = corners[j];
            let area = (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn part2(input: &str) -> u64 {
    let corners = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let (vertical_borders, horizontal_borders) = get_poly_border(&corners);

    let mut max_area = 0;
    for i in 0..corners.len() {
        'nextCorner: for j in i + 1..corners.len() {
            let a @ (x1, y1) = corners[i];
            let b @ (x2, y2) = corners[j];
            let c @ (x3, y3) = (x1, y2);
            let d @ (x4, y4) = (x2, y1);

            // Check if both opposite corners are inside the polygon
            if !is_inside_polygon((x3, y3), &vertical_borders, &horizontal_borders, true)
                || !is_inside_polygon((x4, y4), &vertical_borders, &horizontal_borders, true)
            {
                continue;
            }

            // Check if a different corner lies inside the rectangle
            for &corner @ (cx, cy) in &corners {
                if corner != a
                    && corner != b
                    && cx > x1.min(x2)
                    && cx < x1.max(x2)
                    && cy > y1.min(y2)
                    && cy < y1.max(y2)
                {
                    continue 'nextCorner;
                }
            }

            // Check if the polygon borders intersect with the rectangle
            let vertical_rect_borders = [(a, c), (b, d)];
            let horizontal_rect_borders = [(a, d), (b, c)];

            for &((p1x, p1y), (p2x, _)) in &horizontal_borders {
                let px_s = p1x.min(p2x);
                let px_e = p1x.max(p2x);

                for &((rx1, ry1), (_, ry2)) in &vertical_rect_borders {
                    let ry_s = ry1.min(ry2);
                    let ry_e = ry1.max(ry2);

                    if px_s < rx1 && rx1 < px_e && ry_s < p1y && p1y < ry_e {
                        continue 'nextCorner;
                    }
                }
            }

            for &((p1x, p1y), (_, p2y)) in &vertical_borders {
                let py_s = p1y.min(p2y);
                let py_e = p1y.max(p2y);

                for &((rx1, ry1), (rx2, _)) in &horizontal_rect_borders {
                    let rx_s = rx1.min(rx2);
                    let rx_e = rx1.max(rx2);

                    if py_s < ry1 && ry1 < py_e && rx_s < p1x && p1x < rx_e {
                        continue 'nextCorner;
                    }
                }
            }

            let area = (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

type Point = (u64, u64);
type Border = (Point, Point);

fn get_poly_border(corners: &[(u64, u64)]) -> (Vec<Border>, Vec<Border>) {
    let mut vertical_borders = Vec::new();
    let mut horizontal_borders = Vec::new();
    let mut current = corners[0];
    let mut visited = HashSet::new();

    loop {
        visited.insert(current);

        // Find next corner in the same row or column
        let mut next = None;
        for &corner in corners {
            if corner != current
                && (!visited.contains(&corner))
                && (corner.0 == current.0 || corner.1 == current.1)
            {
                next = Some(corner);
                break;
            }
        }
        if let Some(n) = next {
            if n.0 == current.0 {
                vertical_borders.push((current, n));
            } else {
                horizontal_borders.push((current, n));
            }
            current = n;
        } else {
            break;
        }
    }

    // Connect last with first
    let first = corners[0];
    if first.0 == current.0 {
        vertical_borders.push((current, first));
    } else {
        horizontal_borders.push((current, first));
    }

    (vertical_borders, horizontal_borders)
}

fn is_inside_polygon(
    point: Point,
    vertical_borders_ranges: &[(Point, Point)],
    horizontal_borders_ranges: &[(Point, Point)],
    allow_on_edge: bool,
) -> bool {
    let (px, py) = point;

    // Check if point lies exactly on a horizontal edge
    for &((p1x, p1y), (p2x, _)) in horizontal_borders_ranges {
        if p1y == py {
            let minx = p1x.min(p2x);
            let maxx = p1x.max(p2x);
            if px >= minx && px <= maxx {
                return allow_on_edge;
            }
        }
    }

    // Check if point lies exactly on a vertical edge
    for &((p1x, p1y), (_, p2y)) in vertical_borders_ranges {
        if p1x == px {
            let miny = p1y.min(p2y);
            let maxy = p1y.max(p2y);
            if py >= miny && py <= maxy {
                return allow_on_edge;
            }
        }
    }

    let mut crossings = 0;

    for &((p1x, p1y), (_, p2y)) in vertical_borders_ranges {
        let x0 = p1x;
        let (y1, y2) = (p1y, p2y);

        // Check if segment crosses the horizontal ray
        let miny = y1.min(y2);
        let maxy = y1.max(y2);

        if py >= miny && py < maxy && px < x0 {
            crossings += 1;
        }
    }

    crossings % 2 == 1
}

aoc2025::test!(
    "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
",
    50,
    24
);
