// I see a problem with my solution it is not scalable you could use some nice
// math formular to calculate the res without matching the string
// TODO: fix this code to be scalable with math


/// main function
fn main() {
//     let input = String::from(
//         "A Y
// B X
// C Z
// ",
//     );
    let input = include_str!("./input2.txt");
    // let games_test: Vec<&str> = test_in.split("\n").collect();
    let score = problem_one(&input.clone());
    let score2 = problem_two(&input);
    println!("{}", score);
    println!("{}", score2);
}

/// solves problem one
/// infos:
/// first column:
/// A Rock
/// B Paper
/// C Scissors
/// second column:
/// X Rock Score: 1
/// Y Paper Score: 2
/// Z Scissors Score: 3
fn problem_one(input: &str) -> usize {
    let mut score: usize = 0;
    let mut games: Vec<&str> = input.split("\n").collect();
    // there is a new line in my input file that is shitty hmmm
    games.pop();
    for g in games {
        let game_vec: Vec<&str> = g.split(" ").collect();
        let first = game_vec.get(0).unwrap();
        let second = game_vec.get(1).unwrap();
        match first.to_owned() {
            "A" => match second.to_owned() {
                "X" => { score += 1 + 3},
                "Y" => { score += 2 + 6},
                "Z" => { score += 3 + 0},
                x => panic!("first is not X, Y or Z but {}", x),
            },
            "B" => match second.to_owned() {
                "X" => {score += 1 + 0},
                "Y" => {score += 2 + 3},
                "Z" => {score += 3 + 6},
                x => panic!("first is not X, Y or Z but {}", x),
            },
            "C" => match second.to_owned() {
                "X" => {score += 1 + 6},
                "Y" => {score += 2 + 0},
                "Z" => {score += 3 + 3},
                x => panic!("first is not X, Y or Z but {}", x),
            },
            x => panic!("first is not A, B or C but {}", x),
        }
    }
    return score;
}


/// solves problem two
/// infos:
/// first column:
/// A Rock
/// B Paper
/// C Scissors
/// second column:
/// X Lose
/// Y Draw
/// Z Win
fn problem_two(input: &str) -> usize {
    let mut score: usize = 0;
    let mut games: Vec<&str> = input.split("\n").collect();
    games.pop();
    for g in games {
        let game_vec: Vec<&str> = g.split(" ").collect();
        let first = game_vec.get(0).unwrap();
        let second = game_vec.get(1).unwrap();
        match first.to_owned() {
            "A" => match second.to_owned() {
                "X" => { score += 3 + 0}, // A ? = X; ? = Scissors => 3p + 0p
                "Y" => { score += 1 + 3}, // 
                "Z" => { score += 2 + 6},
                x => panic!("first is not X, Y or Z but {}", x),
            },
            "B" => match second.to_owned() {
                "X" => {score += 1 + 0},
                "Y" => {score += 2 + 3},
                "Z" => {score += 3 + 6},
                x => panic!("first is not X, Y or Z but {}", x),
            },
            "C" => match second.to_owned() {
                "X" => {score += 2 + 0},
                "Y" => {score += 3 + 3},
                "Z" => {score += 1 + 6},
                x => panic!("first is not X, Y or Z but {}", x),
            },
            x => panic!("first is not A, B or C but {}", x),
        }
    }
    return score;
}
