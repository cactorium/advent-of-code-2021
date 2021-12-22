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

    fn intersects(&self, other: &Cuboid) -> bool {
        let dim_intersects = |(a_s, a_e), (b_s, b_e)| {
            ((a_s <= b_s) && (b_s < a_e)) || 
                ((b_s <= a_s) && (a_s < b_e))
        };
        dim_intersects(self.x, other.x) &&
            dim_intersects(self.y, other.y) &&
            dim_intersects(self.z, other.z)

    }

    fn volume(&self) -> isize {
        (self.x.1 - self.x.0)*(self.y.1 - self.y.0)*(self.z.1 - self.z.0)
    }

    // subtracts other from this cuboid,
    // producing a number of cuboids representing the resulting region
    fn difference(&self, other: &Cuboid) -> Vec<Cuboid> {
        let calc_split = |(a_s, a_e), (b_s, b_e)| {
            if a_s <= b_s && b_e <= a_e {
                // b is contained in a, produce three regions
                vec![(a_s, b_s), (b_s, b_e), (b_e, a_e)]
            } else if b_s <= a_s && a_e <= b_e {
                // a is contained in b, produce one region
                vec![(a_s, a_e)]
            } else if a_s <= b_s && a_e < b_e { // overlap with a first
                vec![(a_s, b_s), (b_s, a_e)]
            } else if b_s <= a_s && b_e < a_e { // overlap with b first
                vec![(a_s, b_e), (b_e, a_e)]
            } else { // there's entirely disjoint, return a's single region
                vec![(a_s, a_e)]
            }
        };

        let x_split = calc_split(self.x, other.x);
        let y_split = calc_split(self.y, other.y);
        let z_split = calc_split(self.z, other.z);

        let mut parts = Vec::new();
        for x_reg in x_split.iter() {
            for y_reg in y_split.iter() {
                for z_reg in z_split.iter() {
                    let c = Cuboid {
                        x: *x_reg,
                        y: *y_reg,
                        z: *z_reg,
                    };
                    if !other.contains(&c) && c.volume() > 0 {
                        parts.push(c);
                    }
                }
            }
        }
        parts
    }
}
#[derive(Clone, Copy, Debug)]
enum Command {
    On(Cuboid),
    Off(Cuboid)
}

fn turn_on(cuboids: &mut Vec<Cuboid>, c: &Cuboid) {
    for cu in cuboids.iter() {
        if cu.contains(c) {
            return;
        }
        if cu.intersects(c) {
            let parts = c.difference(cu);
            //println!("split into {:?}", &parts);
            //println!("existing regions {:?}", cuboids);
            for p in parts {
                turn_on(cuboids, &p);
            }
            return;
        }
    }
    cuboids.push(*c);
}

fn turn_off(cuboids: &mut Vec<Cuboid>, c: &Cuboid) {
    // remove any regions contained within c
    let mut toremove: Vec<_> = cuboids.iter().enumerate()
        .filter(|(_, cu)| c.contains(cu))
        .map(|(i, _)| i)
        .collect();
    for i in toremove.iter().rev() {
        cuboids.remove(*i);
    }

    let mut toadd: Vec<Cuboid> = Vec::new();
    for mut cu in cuboids.iter_mut() {
        if cu.intersects(c) {
            let left = cu.difference(c);
            *cu = left[0];
            for part in left[1..].iter() {
                toadd.push(*part);
            }
        }
    }

    for part in toadd {
        cuboids.push(part);
    }
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

    let mut on = Vec::new();

    for c in commands.iter() {
        match c {
            &Command::On(ref c) => {
                turn_on(&mut on, c);
            },
            &Command::Off(ref c) => {
                turn_off(&mut on, c);
            },
        }
        //println!("{:?}", &on);
    }

    let num_on: isize = on.iter().map(|c| c.volume()).sum();

    println!("{}", num_on);
}
