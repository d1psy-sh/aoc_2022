use std::fs;

fn main() {
    let file = "./src/bin/input5.txt";
    // let file = "./src/bin/input5_test.txt";
    let input =
        fs::read_to_string(std::path::Path::new(file)).expect("There was an error while reading");
    let res = problem_one(&input);
    let res_two = problem_two(&input);
    println!("res: {:?}, res_two: {:?}", res, res_two);
}

fn problem_one(input: &String) -> String {
    // define res
    let mut res: String = "".to_string();
    // hard code parse for now
    // [N]     [Q]         [N]
    // [R]     [F] [Q]     [G] [M]
    // [J]     [Z] [T]     [R] [H] [J]
    // [T] [H] [G] [R]     [B] [N] [T]
    // [Z] [J] [J] [G] [F] [Z] [S] [M]
    // [B] [N] [N] [N] [Q] [W] [L] [Q] [S]
    // [D] [S] [R] [V] [T] [C] [C] [N] [G]
    // [F] [R] [C] [F] [L] [Q] [F] [D] [P]
    //  1   2   3   4   5   6   7   8   9
    //  ne struct
    let mut stacks = vec![
        vec!["N", "R", "J", "T", "Z", "B", "D", "F"],
        vec!["H", "J", "N", "S", "R"],
        vec!["Q", "F", "Z", "G", "J", "N", "R", "C"],
        vec!["Q", "T", "R", "G", "N", "V", "F"],
        vec!["F", "Q", "T", "L"],
        vec!["N", "G", "R", "B", "Z", "W", "C", "Q"],
        vec!["M", "H", "N", "S", "L", "C", "F"],
        vec!["J", "T", "M", "Q", "N", "D"],
        vec!["S", "G", "P"],
    ];
    // yes i am a dumbo should have reved it from the start
    // if you cheat cheat right lol :)
    stacks = stacks.into_iter().map(|x| x.into_iter().rev().collect::<Vec<&str>>()).collect();
    let moves = move_parser(&input);
    for m in moves {
        let mut tmp = vec![];
        let source = m[1] - 1;
        let dest = m[2] - 1;
        for _ in 0..m[0] {
            let a = stacks[source as usize].pop().unwrap();
            tmp.push(a);
        }
        for e in tmp {
            stacks[dest as usize].push(e);
        }
    }
    // here we sum up the first of all stacks
    for stack in stacks {
        // take the first
        res.push(stack.last().unwrap().parse().unwrap());
    }
    return res;
}


fn problem_two(input: &String) -> String {
    // define res
    let mut res: String = "".to_string();
    // hard code parse for now
    // [N]     [Q]         [N]
    // [R]     [F] [Q]     [G] [M]
    // [J]     [Z] [T]     [R] [H] [J]
    // [T] [H] [G] [R]     [B] [N] [T]
    // [Z] [J] [J] [G] [F] [Z] [S] [M]
    // [B] [N] [N] [N] [Q] [W] [L] [Q] [S]
    // [D] [S] [R] [V] [T] [C] [C] [N] [G]
    // [F] [R] [C] [F] [L] [Q] [F] [D] [P]
    //  1   2   3   4   5   6   7   8   9
    let mut stacks = vec![
        vec!["N", "R", "J", "T", "Z", "B", "D", "F"],
        vec!["H", "J", "N", "S", "R"],
        vec!["Q", "F", "Z", "G", "J", "N", "R", "C"],
        vec!["Q", "T", "R", "G", "N", "V", "F"],
        vec!["F", "Q", "T", "L"],
        vec!["N", "G", "R", "B", "Z", "W", "C", "Q"],
        vec!["M", "H", "N", "S", "L", "C", "F"],
        vec!["J", "T", "M", "Q", "N", "D"],
        vec!["S", "G", "P"],
    ];
    // yes i am a dumbo should have reved it from the start
    // if you cheat cheat right lol :)
    stacks = stacks.into_iter().map(|x| x.into_iter().rev().collect::<Vec<&str>>()).collect();
    let moves = move_parser(&input);
    for m in moves {
        let mut tmp = vec![];
        let source = m[1] - 1;
        let dest = m[2] - 1;
        for _ in 0..m[0] {
            let a = stacks[source as usize].pop().unwrap();
            tmp.push(a);
        }
        for e in tmp.into_iter().rev() {
            stacks[dest as usize].push(e);
        }
    }
    // here we sum up the first of all stacks
    for stack in stacks {
        // take the first
        res.push(stack.last().unwrap().parse().unwrap());
    }
    return res;
    
}

fn move_parser(input: &String) -> Vec<Vec<u8>> {
    let moves = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .collect::<Vec<&str>>();

    let mut vectors: Vec<Vec<u8>> = vec![];
    for item in moves {
        let mut vector: Vec<u8> = vec![];
        for element in item.split(" ") {
            let opt = element.parse::<u8>();
            match opt {
                Ok(ok) => vector.push(ok),
                Err(_) => (),
            }
        }
        vectors.push(vector);
    }
    // pop the last cuz is new line without a new move
    vectors.pop();
    return vectors;
}

/// this is the parse stacks function it takes the input
/// and parses out a vec of vec of strings from the stacks
fn parse_stacks(input: String) -> Vec<Vec<String>> {
    let stacks: Vec<Vec<String>> = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_string()
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| {
            x.split(" ")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        })
        .collect();
    // for string in stacks
    // for iter take 4 or 3 if last
    // vec of vecs push letter in []

    // return vec of vecs
    return stacks;
}
