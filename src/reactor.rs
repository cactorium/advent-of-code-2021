use std::collections::HashSet;
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
struct Cuboid {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Cuboid {
    fn contains(&self, other: &Cuboid) -> bool {
        let dim_contains = |(a_s, a_e), (b_s, b_e)| {
            (a_s <= b_s) && (b_e <= a_e)
        };
        dim_contains(self.x, other.x) &&
            dim_contains(self.y, other.y) &&
            dim_contains(self.z, other.z)
    }

    fn volume(&self) -> isize {
        (self.x.1 - self.x.0)*(self.y.1 - self.y.0)*(self.z.1 - self.z.0)
    }
}
#[derive(Clone, Copy, Debug)]
enum Command {
    On(Cuboid),
    Off(Cuboid)
}

fn main() {
    let mut commands: Vec<Command> = Vec::new();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        let parts: Vec<_> = line.split(" ").collect();
        let coords: Vec<_> = parts[1].split(",").collect();
        if coords.len() != 3 {
            panic!("invalid string {}, could not parse parts", line);
        }
        let (x, y, z) = {
            let cs: Vec<_> = coords.iter().zip(["x=", "y=", "z="].iter())
                .map(|(s, prefix)| if !s.starts_with(prefix) {
                    panic!("invalid string {}, did not start with {}", s, prefix);
                } else {
                    let trimmed = &s[prefix.len()..];
                    trimmed.split("..")
                        .map(|t| str::parse::<isize>(t).unwrap())
                        .collect::<Vec<isize>>()
                })
                .collect();
            // +1 to convert from inclusive end range to exclusive end range
            ((cs[0][0], cs[0][1] + 1), (cs[1][0], cs[1][1] + 1), (cs[2][0], cs[2][1] + 1))
        };
        let cuboid = Cuboid {
            x: x,
            y: y,
            z: z
        };
        if parts[0] == "on" {
            commands.push(Command::On(cuboid));
        } else {
            commands.push(Command::Off(cuboid));
        }
    }

    println!("{:?}", &commands);

    let mut x_pts = HashSet::new();
    let mut y_pts = HashSet::new();
    let mut z_pts = HashSet::new();
    for c in commands.iter() {
        match c {
            &Command::On(c) => {
                x_pts.insert(c.x.0);
                x_pts.insert(c.x.1);
                y_pts.insert(c.y.0);
                y_pts.insert(c.y.1);
                z_pts.insert(c.z.0);
                z_pts.insert(c.z.1);
            },
            &Command::Off(c) => {
                x_pts.insert(c.x.0);
                x_pts.insert(c.x.1);
                y_pts.insert(c.y.0);
                y_pts.insert(c.y.1);
                z_pts.insert(c.z.0);
                z_pts.insert(c.z.1);
            },
        }
    }

    let mut x_vec: Vec<isize> = x_pts.into_iter().collect();
    x_vec.sort();
    let x_sorted = x_vec;
    let mut y_vec: Vec<isize> = y_pts.into_iter().collect();
    y_vec.sort();
    let y_sorted = y_vec;
    let mut z_vec: Vec<isize> = z_pts.into_iter().collect();
    z_vec.sort();
    let z_sorted = z_vec;

    let mut num_on: isize = 0;
    let bounds = Cuboid {
        x: (-500, 501),
        y: (-500, 501),
        z: (-500, 501),
    };
    for (xs, xe) in x_sorted.iter().zip(x_sorted.iter().skip(1)) {
        for (ys, ye) in y_sorted.iter().zip(y_sorted.iter().skip(1)) {
            for (zs, ze) in z_sorted.iter().zip(z_sorted.iter().skip(1)) {
                let mut on = false;
                let cur_cuboid = Cuboid {
                    x: (*xs, *xe),
                    y: (*ys, *ye),
                    z: (*zs, *ze),
                };
                if !bounds.contains(&cur_cuboid) {
                    continue;
                }

                for c in commands.iter() {
                    match c {
                        &Command::On(c) => {
                            if c.contains(&cur_cuboid) {
                                on = true;
                            }
                        },
                        &Command::Off(c) => {
                            if c.contains(&cur_cuboid) {
                                on = false;
                            }
                        },
                    }
                }
                if bounds.contains(&cur_cuboid) {
                    if on {
                        num_on += cur_cuboid.volume();
                    }
                }
            }
        }
    }

    println!("{}", num_on);
}
