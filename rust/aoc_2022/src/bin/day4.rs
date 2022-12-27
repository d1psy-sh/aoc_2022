fn main() {
    // let input = include_str!("./input4_test.txt");
    let input = include_str!("./input4.txt");
    let score_one = problem_one(input);
    println!("{}", score_one);
    let score_two = problem_two(input);
    println!("{}", score_two);
}

fn problem_one(input: &str) -> usize {
    let mut score: usize = 0;
    let mut pairs = input.split("\n").collect::<Vec<&str>>();
    pairs.pop();
    for p in pairs {
        let tmp: Vec<&str> = p.split(",").collect();
        for a in tmp.chunks(2).into_iter() {
            let numbers_one = a[0]
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let numbers_two = a[1]
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            score += if (numbers_one[0] >= numbers_two[0] && numbers_one[1] <= numbers_two[1]) || (numbers_one[0] <= numbers_two[0] && numbers_one[1] >= numbers_two[1]) {
                1
            } else {
                0
            };
        }
    }
    return score;
}

fn problem_two(input: &str) -> usize {
    let mut score: usize = 0;
    let mut pairs = input.split("\n").collect::<Vec<&str>>();
    pairs.pop();
    for p in pairs {
        let tmp: Vec<&str> = p.split(",").collect();
        for a in tmp.chunks(2).into_iter() {
            let numbers_one = a[0]
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let numbers_two = a[1]
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            // just have to change the conditions
            score += if numbers_one[0] >= numbers_two[0] && numbers_one[1] <= numbers_two[1] {
                1
            } else if numbers_one[0] <= numbers_two[0] && numbers_one[1] >= numbers_two[1] {
                1
            } else if numbers_one[1] >= numbers_two[0] && numbers_one[0] <= numbers_two[1] {
                1
            } else {
                0
            };
        }
    }
    return score;
}
