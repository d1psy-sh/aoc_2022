fn main() {
    let input = include_str!("./input3.txt");
    // let input = include_str!("./input3_test.txt");
    let score1 = problem_one(&input);
    let score2 = problem_two(&input);

    println!("{}", score1);
    println!("{}", score2);
    // here comes some implementation
}

fn problem_two(input: &str) -> usize {
    let mut score: usize = 0;
    
    let mut backpacks: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    backpacks.pop();

    for b in backpacks.chunks(3) {
        for letter in b[0].chars() {
            if b[1].contains(letter) && b[2].contains(letter) {
                score += get_score(letter);
                break;
            } else {
                continue;
            }
        }
    }
    
    return score;
}

fn problem_one(input: &str) -> usize {
    
    let mut backpacks: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    backpacks.pop();
    let mut score: usize = 0;
    for b in backpacks {
        let half_size: usize = match b.len() {
            0 => panic!("this is empty"),
            s => s / 2,
        };
        let backpack = vec![&b[0..half_size], &b[half_size..]];
        for letter in backpack[0].chars() {
            let item = if backpack[1].contains(letter) {
                Some(letter)
            } else {
                None
            };
            match item {
                Some(item) => {
                    score += get_score(item);
                    break;
                }
                None => continue,
            }
        }
    }
    return score;
}

fn get_score(item: char) -> usize {
    if item.is_uppercase() {
        (item as usize - 'A' as usize) + 27
    } else {
        item as usize - 'a' as usize + 1
    }
}
