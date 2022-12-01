fn main() {
    let input = include_str!("./input.txt");
    let split = input.split("\n\n").collect::<Vec<&str>>();
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

    println!("{:?}", res);
}
