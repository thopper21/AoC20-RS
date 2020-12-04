#[derive(Clone)]
struct Slope {
    dx: usize,
    dy: usize,
    x: usize,
    y: usize,
    trees: usize,
}

impl Slope {
    fn new(dx: usize, dy: usize) -> Slope {
        Slope {
            dx,
            dy,
            x: 0,
            y: 0,
            trees: 0,
        }
    }
}

fn run<I>(input: I, starts: Vec<Slope>) -> usize
where
    I: Iterator<Item = String>,
{
    input
        .fold(starts, |slopes, line| {
            let len = line.chars().count();

            slopes
                .into_iter()
                .map(|slope| {
                    let mut next = slope.clone();
                    if next.y % next.dy == 0 {
                        if next.x >= len {
                            next.x %= len;
                        }

                        if line.chars().nth(next.x) == Some('#') {
                            next.trees += 1;
                        }

                        next.x += next.dx;
                    }
                    next.y += 1;

                    next
                })
                .collect()
        })
        .into_iter()
        .fold(1, |x, slope| {
            return x * slope.trees;
        })
}

pub fn part1<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    run(input, vec![Slope::new(3, 1)])
}

pub fn part2<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    run(
        input,
        vec![
            Slope::new(1, 1),
            Slope::new(3, 1),
            Slope::new(5, 1),
            Slope::new(7, 1),
            Slope::new(1, 2),
        ],
    )
}
