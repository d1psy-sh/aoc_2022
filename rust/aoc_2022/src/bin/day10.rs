use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/input9.txt").unwrap();
    // let input = fs::read_to_string("./src/bin/input9_test.txt").unwrap();
    let res = problem_one(&input);
    println!("Problem one: {}", res);
    problem_two(&input);
}

fn problem_one(input: &String) -> isize {
    let mut res = 0;
    let commands = input.lines();
    let mut reg = 1;
    let mut i = 0;
    for command in commands {
        i += 1;
        if i == 20 || i == 60 || i == 100 || i == 140 || i == 180 || i == 220 {
            res += dbg!(i * reg);
        }
        match command {
            "noop" => (),
            _ => {
                i += 1;
                if i == 20 || i == 60 || i == 100 || i == 140 || i == 180 || i == 220 {
                    res += dbg!(i * reg);
                }
                reg += command
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .parse::<isize>()
                    .unwrap();
            }
        }
    }
    return res;
}

fn problem_two(input: &String) {
    let commands = input.lines();
    let mut reg = 1;
    let mut i = 0;
    let mut crt: String = String::from("");
    for command in commands {
        i += 1;
        if (i-1) % 40 == 0 {
            crt += "\n";
            i = 1;
        }
        if i == reg || i == reg + 1 || i == reg + 2 {
            crt += "#";
        } else {
            crt += ".";
        }
        match command {
            "noop" => (),
            _ => {
                i += 1;
                if (i-1) % 40 == 0 {
                    crt += "\n";
                    i = 1;
                }
                if i == reg || i == reg + 1 || i == reg + 2 {
                    crt += "#";
                } else {
                    crt += ".";
                }
                reg += command
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .parse::<isize>()
                    .unwrap();
            }
        }
    }
    println!("{}", crt);
}
