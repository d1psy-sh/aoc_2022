use std::fs;

fn main() {
    let path = std::path::Path::new("./src/bin/input8.txt");
    // let path = std::path::Path::new("./src/bin/input8_test.txt");
    let input = fs::read_to_string(path).unwrap();
    let res: usize = problem_one(&input);
    let res_two: usize = problem_two(&input);
    println!("res: {res}, res_two: {res_two}");
}

#[derive(Debug, Clone, Copy)]
struct Punkt {
    x: usize,
    y: usize,
}

impl Punkt {
    fn go_right(&mut self) {
        self.x += 1;
    }
    fn go_left(&mut self) {
        self.x -= 1;
    }
    fn go_up(&mut self) {
        self.y += 1;
    }
    fn go_down(&mut self) {
        self.y -= 1;
    }
    fn dir_walk(& mut self, dir: &str) {
        match dir {
            "R" => self.go_right(),
            "L" => self.go_left(),
            "U" => self.go_up(),
            "D" => self.go_down(),
            _ => panic!("Unknown direction"),
        }
    }
}

// fn walk<'a>(
//     dir: &str,
//     p: &mut Punkt,
//     seen: &'a mut Vec<Vec<bool>>,
//     matrix: &Vec<Vec<usize>>,
//     highest: usize,
// ) -> Vec<Vec<bool>> {
//     // dbg!(&highest);
//     // stop recursion
//     if p.x == 0 || p.y == 0 || p.x == matrix[0].len() - 1 || p.y == matrix.len() - 1 {
//         // println!("Out of bounds");
//         return seen.to_owned();
//     }
//     if matrix[p.y][p.x] > highest {
//         // println!("hello2");
//         seen[p.y][p.x] = true;
//         return walk(dir, &mut p.dir_walk(dir), seen, matrix, matrix[p.y][p.x]);
//     }
//     // println!("hello");
//     return walk(dir, &mut p.dir_walk(dir), seen, matrix, highest);
// }

fn problem_one(input: &String) -> usize {
    let mut res = 0;
    let mut matrix: Vec<Vec<usize>> = input
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    // dbg!(&matrix);
    matrix.pop();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; matrix[0].len()]; matrix.len()];
    // edges of matrix should be true
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == 0 || j == 0 || i == matrix.len() - 1 || j == matrix[i].len() - 1 {
                seen[i][j] = true;
            }
        }
    }
    // go from the top every row and check whether the val is the greatest val
    // since
    for i in 1..matrix[0].len() {
        let mut temp = matrix[0][i];
        // that is without matrix.len()
        for j in 0..matrix.len() {
            if matrix[j][i] > temp {
                seen[j][i] = true;
                temp = matrix[j][i];
            }
        }
    }
    for i in 1..matrix[0].len() {
        let top = matrix.len() - 1;
        let mut temp = matrix[top][i];
        for j in 0..matrix.len() {
            if matrix[top - j][i] > temp {
                seen[top - j][i] = true;
                temp = matrix[top - j][i];
            }
        }
    }
    for i in 1..matrix.len() {
        let mut temp = matrix[i][0];
        for j in 0..matrix[0].len() {
            if matrix[i][j] > temp {
                seen[i][j] = true;
                temp = matrix[i][j];
            }
        }
    }
    for i in 1..matrix.len() {
        let mut temp = matrix[i][matrix[0].len() - 1];
        for j in 0..matrix[0].len() {
            if matrix[i][matrix[0].len() - 1 - j] > temp {
                seen[i][matrix[0].len() - 1 - j] = true;
                temp = matrix[i][matrix[0].len() - 1 - j];
            }
        }
    }
    // sum the true vals in the seen
    for i in 0..seen.len() {
        for j in 0..seen[i].len() {
            if seen[i][j] {
                res += 1;
            }
        }
    }
    return res;
}

fn get_score_dir(dir: &str, punkt: &Punkt, val: usize, matrix: &Vec<Vec<usize>>) -> usize {
    let mut p = punkt.clone();
    p.dir_walk(dir);
    let mut dir_score = 1;

    while (p.y != 0) && (p.y < (matrix.len() - 1)) && (p.x != 0) && (p.x < (matrix[0].len()) - 1) {
        if matrix[p.y][p.x] < val {
            // dbg!(&p);
            dir_score += 1;
            p.dir_walk(dir);
        } else {
            break;
        }
    }
    return dir_score;
}

fn scenic_score(p: Punkt, val: usize, matrix: &Vec<Vec<usize>>) -> usize {
    if p.y == 0 || p.x == 0 || p.x == matrix[0].len() - 1 || p.y == matrix.len() - 1 {
        return 0;
    }
    let scores = vec![
        get_score_dir("D", &p, val, matrix),
        get_score_dir("U", &p, val, matrix),
        get_score_dir("L", &p, val, matrix),
        get_score_dir("R", &p, val, matrix),
    ];
    // dbg!(&scores);
    // product of scores is score
    let score = scores.iter().product();
    return score;
}

fn problem_two(input: &String) -> usize {
    let mut matrix: Vec<Vec<usize>> = input
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    // dbg!(&matrix);
    matrix.pop();
    let mut seen: Vec<Vec<usize>> = vec![vec![0; matrix[0].len()]; matrix.len()];
    // edges of matrix should be true
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            seen[i][j] = scenic_score(Punkt { x: j, y: i }, matrix[i][j], &matrix);
        }
    }
    // let res equal the max of the seen
    return seen.iter().flatten().max().unwrap().clone();
}
