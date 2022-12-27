use std::fs;

fn main() {
    let path = std::path::Path::new("./src/bin/input8.txt");
    let input = fs::read_to_string(path).unwrap();
    let res: usize = problem_one(&input);

    let res_two: usize = problem_two(&input);
    println!("res: {res}, res_two: {res_two}");
}

struct Punkt {
    x: i32,
    y: i32,
}

impl Punkt {
    fn go_right(&self) -> Punkt {
        Punkt {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn go_left(&self) -> Punkt {
        Punkt {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn go_up(&self) -> Punkt {
        Punkt {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn go_down(&self) -> Punkt {
        Punkt {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn dir_walk(&self, dir: &str) -> Punkt {
        match dir {
            "R" => self.go_right(),
            "L" => self.go_left(),
            "U" => self.go_up(),
            "D" => self.go_down(),
            _ => panic!("Unknown direction"),
        }
    }
}

fn walk(dir: &str, p: Punkt, seen: &mut Vec<Vec<bool>>) {
    // dont know if this is right
    // TODO: investigate
    if seen[p.y as usize][p.x as usize] {
        walk(dir, p.dir_walk(dir), seen);
    }
}

fn problem_one(input: &String) -> usize {
    let res = 0;
    let matrix: Vec<&str> = input.split("\n").collect();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; matrix.len()]; matrix.len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == 0 || j == 0 || i == matrix.len() - 1 || j == matrix.len() - 1 {
                seen[i][j] = true;
            }
        }
    }
    for i in 1..matrix.len() - 1 {
        walk("D", Punkt { x: i as i32, y: 1 }, &mut seen);
    }
    return res;
}

fn problem_two(input: &String) -> usize {
    let res = 0;
    return res;
}
