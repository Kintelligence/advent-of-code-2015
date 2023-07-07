fn main() {
    let total = std::time::SystemTime::now();
    let mut timer = std::time::SystemTime::now();
    let input = parse("input.txt");
    println!("Parse: {:?}", timer.elapsed());
    timer = std::time::SystemTime::now();
    let result = solve(&input, 3);
    println!("Part 1: {:?}", timer.elapsed());
    println!("{:?}", result);
    timer = std::time::SystemTime::now();
    let result = solve(&input, 4);
    println!("Part 2: {:?}", timer.elapsed());
    println!("{:?}", result);
    println!("Total: {:?}", total.elapsed());
}

fn parse(file: &str) -> Vec<usize> {
    let path = std::env::current_dir()
        .expect("expected directory")
        .join(std::path::Path::new(file));

    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.trim().parse().expect("expect integer"))
        .collect()
}

fn solve(input: &Vec<usize>, div: usize) -> usize {
    let target: usize = input.iter().sum::<usize>() / div;
    let mut solutions: Vec<Group> = vec![Group {
        amount: 0,
        weight: 0,
        entanglement: 1,
    }];
    let mut best: Group = Group {
        amount: usize::MAX,
        weight: usize::MAX,
        entanglement: usize::MAX,
    };
    let mut prune: bool;

    for i in input.iter().rev() {
        prune = false;
        let mut next: Vec<Group> = Vec::new();

        for s in solutions.iter() {
            if s.amount >= best.amount {
                continue;
            }

            let mut n = s.clone();

            n.amount += 1;
            n.weight += i;
            n.entanglement *= i;

            if n.weight > target {
                continue;
            }

            if n.entanglement >= best.entanglement {
                continue;
            }

            if n.weight == target {
                best = n;
                prune = true;
                continue;
            }

            next.push(n);
        }

        solutions.append(&mut next);
        if prune {
            solutions.retain(|group| {
                group.amount < best.amount && group.entanglement < best.entanglement
            });
        }
    }

    return best.entanglement;
}

#[derive(Clone, Copy)]
struct Group {
    amount: usize,
    weight: usize,
    entanglement: usize,
}
