use std::fs;

fn main() {
    // let input = fs::read_to_string("./src/bin/input11.txt").expect("Error reading input.txt");
    let input = fs::read_to_string("./src/bin/input11_test.txt").expect("Error reading input.txt");
    let res = problem_one(&input);
    println!("Result: {}", res);
}

#[derive(Debug, Clone)]
struct Monkey {
    num: u32,
    items: Vec<usize>,
    rest: Vec<String>,
}

fn problem_one(input: &String) -> usize {
    let mut res = 0;
    // Monkey 0:
    // Starting items: 79, 98
    // Operation: new = old * 19
    // Test: divisible by 23
    //  If true: throw to monkey 2
    //  If false: throw to monkey 3
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|x| {
            let lines = x.split("\n").collect::<Vec<&str>>();
            let monkey = Monkey {
                num: lines[0].split(" ").collect::<Vec<&str>>()[1]
                    .trim_end_matches(":")
                    .parse::<u32>()
                    .unwrap(),
                items: lines[1].split(" ").collect::<Vec<&str>>()[3]
                    .split(",").map(|y| y.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
                rest: lines[2..].iter().map(|x| x.to_string()).collect(),
            };

            return monkey;
        })
        .collect();
    println!("{:?}", monkeys);

    return res;
}
