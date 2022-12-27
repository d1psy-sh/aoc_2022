use std::fs;

fn main() {
    // let path = std::path::Path::new("./src/bin/input7.txt");
    let path = std::path::Path::new("./src/bin/input7_test.txt");
    let input = fs::read_to_string(path).unwrap();

    let res: usize = problem_one(&input);
    let res_two: usize = problem_one(&input);
    println!("res: {res}, res_two: {res_two}");
}

fn problem_one(input: &String) -> usize {
    return 0;
}

fn problem_two(input: &String) -> usize {
    return 0;
}
