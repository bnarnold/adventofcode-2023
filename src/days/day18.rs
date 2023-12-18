use rayon::prelude::*;

fn level1(input: &str) -> i64 {
    let edges = input.trim().par_lines().map(|line| {
        let mut chunks = line.split_whitespace();
        let dir = chunks.next().expect("No dir");
        let delta: i64 = chunks
            .next()
            .expect("No length")
            .parse()
            .expect("length not a number");
        match dir {
            "R" => (delta, 0),
            "D" => (0, delta),
            "L" => (-delta, 0),
            "U" => (0, -delta),
            _ => unimplemented!("{dir}"),
        }
    });
    enclosed_area(edges)
}

fn level2(input: &str) -> i64 {
    let edges = input.trim().par_lines().map(|line| {
        let mut chunks = line.split_whitespace();
        let hex = i64::from_str_radix(
            chunks
                .nth(2)
                .expect("No hex code")
                .strip_prefix("(#")
                .expect("No open paren")
                .strip_suffix(")")
                .expect("no close paren"),
            16,
        )
        .expect("not a hex number");
        let dir = match hex & 0xF {
            0 => "R",
            1 => "D",
            2 => "L",
            3 => "U",
            _ => unimplemented!("{hex:x}"),
        };
        let delta = hex >> 4;
        match dir {
            "R" => (delta, 0),
            "D" => (0, delta),
            "L" => (-delta, 0),
            "U" => (0, -delta),
            _ => unimplemented!("{dir}"),
        }
    });
    enclosed_area(edges)
}

fn enclosed_area(edges: impl ParallelIterator<Item = (i64, i64)>) -> i64 {
    let (area, length, _, _) = edges.fold(|| (0, 0, 0, 0), |(area, length, x, y), (dx, dy)| (
    area + y * dx,
    length + dx.abs() + dy.abs(),
    x + dx,
    y + dy,
    )).reduce(|| (0, 0, 0, 0), |(area_1, length_1, x_1, y_1), (area_2, length_2, x_2, y_2)| (
    area_1 + area_2 + y_1 * x_2,
    length_1 + length_2,
    x_1 + x_2,
    y_1 + y_2,
    ));
    area.abs() + length / 2 + 1
}

fn main() {
    let input = r"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
    let now = std::time::Instant::now();
    println!("{}", level1(input));
    println!("{}", level2(input));
    println!("{:?}", now.elapsed());
}
