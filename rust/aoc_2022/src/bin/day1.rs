fn main() {
    let input = include_str!("./input.txt");
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let res_one = problem_one(split.clone());
    let res_two = problem_two(split);
    println!("{:?}", res_one);
    println!("{:?}", res_two);
}

fn problem_one(split: Vec<&str>) -> usize {
    let mut res = 0;
    for s in split {
        let tmp: usize = s
            .split("\n")
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<usize>>()
            .iter()
            .sum();
        if res < tmp {
            res = tmp;
        }
    }
    return res;
}


fn problem_two(split: Vec<&str>) -> usize {
    let mut res: Vec<usize> = vec![]; 
    for s in split {
        let tmp: usize = s
            .split("\n")
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<usize>>()
            .iter()
            .sum();
        res.push(tmp);
    }
    res.sort_by(|a, b| b.cmp(a));
    return res.iter().take(3).sum();
}
