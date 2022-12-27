use std::fs;

fn main() {
    let path = std::path::Path::new("./src/bin/input6.txt");
    let input = fs::read_to_string(path).unwrap();
    let res = problem_one(dbg!(&input));
    let res_two = problem_two(&input);
    println!("res: {res}, res_two: {res_two}");
}

fn problem_one(input: &String) -> usize {
    let res: usize = 0;
    // make a window
    for a in 0..(input.len() - 4) {
        // for every element in the window
        // for every other element in the window
        let mut double = false;
        for i in a..a + 4 {
            for j in a..a + 4 {
                // if there is already a double break
                // if the first element ist the second element continue
                if i == j {
                    continue;
                }
                if input.chars().nth(i).unwrap() == input.chars().nth(j).unwrap() {
                    double = true;
                }
            }
        }
        if double != true {
            return a + 4;
        }
    }
    return res;
}

fn problem_two(input: &String) -> usize {
    let res: usize = 0;
    for a in 0..(input.len() - 14) {
        let mut double = false;
        // we go throught 14 * 14 = 196 elements
        for i in a..a + 14 {
            for j in a..a + 14 {
                if i == j {
                    continue;
                }
                if input[i..i + 1] == input[j..j + 1] {
                    double = true;
                }
            }
        }
        if double != true {
            return a + 14;
        }
    }
    return res;
}
