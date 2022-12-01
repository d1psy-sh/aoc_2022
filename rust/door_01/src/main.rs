use std::fs;
/// because I am lost in rust I will go through this procedurally
fn main() {
    let test_cals: String = String::from(
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
    );
    let test_array = test_cals.split("\n");
    let res = most_cals(test_array);
    println!("the res is: {}", res);

    let input = fs::read_to_string("input.txt").expect("err reading file");
    let cals = input.split("\n");
    let real_res = most_cals(cals.clone());
    println!("the res is: {}", real_res);

    let res2 = most_cals_3(cals);

    println!("the res is: {}", res2);


}

fn most_cals_3(split: std::str::Split<&str>) -> usize {
    let mut res: usize = 0;
    let mut res1: usize = 0;
    let mut res2: usize = 0;
    let mut sum: usize = 0;
    for cal in split {
        if cal == "" {
            if res < sum {
                res2 = res1;
                res1 = res;
                res = sum;
                sum = 0;
                continue;
            }
            if res1 < sum {
                res2 = res1;
                res1 = sum;
                sum = 0;
                continue;
            }
            if res2 < sum {
                res2 = sum;
                sum = 0;
                continue;
            }
            sum = 0;
            continue;
        }
        sum += cal.parse::<usize>().unwrap()
    }
    return res + res1 + res2;
}

fn most_cals(split: std::str::Split<&str>) -> usize {
    let mut res: usize = 0;
    let mut sum: usize = 0;
    for cal in split {
        if cal == "" {
            if res < sum {
                res = sum;
                sum = 0;
                continue;
            }
            sum = 0;
            continue;
        }
        sum += cal.parse::<usize>().unwrap()
    }
    return res;
}
