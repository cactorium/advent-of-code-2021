use std::collections::HashSet;
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
enum Fold {
    V(isize),
    H(isize)
}

fn main() {
    let stdin = std::io::stdin();
    let mut dots: Vec<(isize, isize)> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();
    let mut is_dot = true;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() == 0 {
            is_dot = false;
            continue;
        }

        if is_dot {
            let parts: Vec<_> = line.split(",").collect();
            let x: isize = str::parse(parts[0]).unwrap();
            let y: isize = str::parse(parts[1]).unwrap();
            dots.push((x, y));
        } else {
            if !line.starts_with("fold along ") {
                unreachable!("invalid input, not a fold: {}", line);
            }
            let trimmed = &line["fold along ".len()..line.len()];
            if trimmed.starts_with("x=") {
                folds.push(Fold::H(str::parse(&trimmed[2..trimmed.len()]).unwrap()));
            } else {
                folds.push(Fold::V(str::parse(&trimmed[2..trimmed.len()]).unwrap()));
            }
        }
    }
    println!("{:?}", &dots);
    println!("{:?}", &folds);

    let mut occupied: HashSet<(isize, isize)> = HashSet::new();
    for f in folds.iter() {
        let mut new_dots: Vec<(isize, isize)> = Vec::new();
        occupied.clear();
        for &(ref x, ref y) in &dots {
            let (nx, ny) = match *f {
                Fold::H(fx) => {
                    if *x <= fx {
                        (*x, *y)
                    } else {
                        (2*fx - *x, *y)
                    }
                },
                Fold::V(fy) => {
                    if *y <= fy {
                        (*x, *y)
                    } else {
                        (*x, 2*fy - *y)
                    }
                }
            };
            if !occupied.contains(&(nx, ny)) {
                new_dots.push((nx, ny));
                occupied.insert((nx, ny));
            }
        }
        dots = new_dots;
        //println!("{:?}", &dots);
        //println!("{:?}", dots.len());
    }
    let min_x = dots.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = dots.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = dots.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = dots.iter().map(|&(_, y)| y).max().unwrap();

    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            if !occupied.contains(&(x, y)) {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}
