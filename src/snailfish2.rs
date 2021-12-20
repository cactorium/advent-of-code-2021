use std::io::BufRead;
use std::str;

#[derive(Clone, Debug)]
enum Num {
    Num(u8),
    Pair(Box<Num>, Box<Num>)
}

impl Num {
    fn is_pair(&self) -> bool {
        match self {
            &Num::Num(_) => false,
            &Num::Pair(_, _) => true
        }
    }

    fn mag(&self) -> u64 {
        match self {
            &Num::Num(v) => v as u64,
            &Num::Pair(ref l, ref r) => 3*l.mag() + 2*r.mag(),
        }
    }
}

fn parse_num(s: &mut &[u8]) -> Option<Num> {
    const OPEN_BRACKET: u8 = '[' as u8;
    match s[0] {
        v if v >= ('0' as u8) && v <= ('9' as u8) => {
            *s = &s[1..];
            Some(Num::Num(v - ('0') as u8))
        },
        OPEN_BRACKET => {
            *s = &s[1..];
            let left = parse_num(s);
            if left.is_none() {
                return None;
            }
            if s[0] != (',' as u8) {
                println!("expected comma, saw {}", str::from_utf8(s).unwrap());
                return None;
            }
            *s = &s[1..];

            let right = parse_num(s);
            if right.is_none() {
                return None;
            }

            if s[0] != (']' as u8) {
                println!("expected closing bracket, saw {}", str::from_utf8(s).unwrap());
                return None;
            }
            *s = &s[1..];
            Some(Num::Pair(Box::new(left.unwrap()), Box::new(right.unwrap())))
        },
        _ => {
            println!("expected number or opening bracket, saw {}", str::from_utf8(s).unwrap());
            None
        }
    }
}

#[derive(Debug)]
enum Explode {
    LR(u8, u8), // found a pair that needs to be exploded
    L(u8), // left needs to be propagated
    R(u8), // right needs to be propagated
    D // explosion was done; stop searching
}

fn add_leftmost(num: &mut Num, v: u8) {
    match num {
        &mut Num::Pair(ref mut left, _) => {
            add_leftmost(&mut *left, v);
        },
        &mut Num::Num(ref mut val) => {
            *val += v;
        }
    }
}

fn add_rightmost(num: &mut Num, v: u8) {
    match num {
        &mut Num::Pair(_, ref mut right) => {
            add_rightmost(&mut *right, v);
        },
        &mut Num::Num(ref mut val) => {
            *val += v;
        }
    }
}

// try to find a pair to explode, return true if something was exploded
fn try_explode(num: &mut Num, depth: usize) -> Option<Explode> {
    if depth >= 3 {
        if let &mut Num::Pair(ref mut left, ref mut right) = num {
            if left.is_pair() {
                let old_left: Num = (**left).clone();
                *left = Box::new(Num::Num(0));
                if let Num::Pair(l, r) = old_left {
                    match (*l, *r) {
                        (Num::Num(l), Num::Num(r)) => {
                            add_leftmost(right, r);
                            Some(Explode::L(l))
                        },
                        _ => unreachable!("number is nested too deep {:?}", &num)
                    }
                } else {
                    unreachable!("number is nested too deep {:?}", &num)
                }
            } else if right.is_pair() {
                let old_right: Num = (**right).clone();
                *right = Box::new(Num::Num(0));
                if let Num::Pair(l, r) = old_right {
                    match (*l, *r) {
                        (Num::Num(l), Num::Num(r)) => {
                            add_rightmost(left, l);
                            Some(Explode::R(r))
                        },
                        _ => unreachable!("number is nested too deep {:?}", &num)
                    }
                } else {
                    unreachable!("number is nested too deep {:?}", &num)
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        if let &mut Num::Pair(ref mut left, ref mut right) = num {
            match try_explode(&mut *left, depth + 1) {
                Some(Explode::LR(l, r)) => {
                    // propagate r into the leftmost pair
                    // of the right branch
                    // return L to the higher up to see if it can go somewhere
                    println!("add_leftmost {:?} {}", &right, r);
                    add_leftmost(&mut *right, r);
                    return Some(Explode::L(l));
                },
                Some(Explode::L(l)) => {
                    return Some(Explode::L(l));
                },
                Some(Explode::R(r)) => {
                    add_leftmost(&mut *right, r);
                    return Some(Explode::D);
                },
                Some(Explode::D) => {
                    return Some(Explode::D);
                },
                None => {}
            }
            // check the right side if something hasn't exploded yet
            match try_explode(&mut *right, depth + 1) {
                Some(Explode::LR(l, r)) => {
                    // propagate r into the rightmost pair
                    // of the left branch
                    // return R to the higher up to see if it can go somewhere
                    add_rightmost(&mut *left, l);
                    Some(Explode::R(r))
                },
                Some(Explode::L(l)) => {
                    add_rightmost(&mut *left, l);
                    Some(Explode::D)
                },
                Some(Explode::R(r)) => Some(Explode::R(r)),
                Some(Explode::D) => Some(Explode::D),
                None => None,
            }
        } else {
            None
        }
    }
}

fn try_split(num: &mut Num) -> bool {
    match num {
        &mut Num::Pair(ref mut left, ref mut right) => {
            try_split(left) || try_split(right)
        }
        &mut Num::Num(ref v) => {
            if *v >= 10 {
                *num = Num::Pair(
                    Box::new(Num::Num(v/2)),
                    Box::new(Num::Num((v + 1)/2)));
                true
            } else {
                false
            }
        }
    }
}

fn reduce(num: &mut Num) {
    let mut changed = true;
    while changed {
        //println!("{:?}", &num);
        if try_explode(num, 0).is_some() {
            changed = true;
            continue;
        }
        changed = try_split(num);
    }
}

fn concat(a: Num, b: Num) -> Num {
    let mut ret = Num::Pair(Box::new(a), Box::new(b));
    reduce(&mut ret);
    ret
}

fn main() {
    let stdin = std::io::stdin();

    let numbers: Vec<Num> = stdin.lock().lines().map(|line| {
        let s = line.unwrap();
        let s = s.trim();
        let s = s.as_bytes();
        let mut sp: &[u8] = &s;
        parse_num(&mut sp).unwrap()
    }).collect();

    let mut max = 0;
    for (i, a) in numbers.iter().enumerate() {
        for (j, b) in numbers.iter().enumerate() {
            if i == j {
                continue;
            }

            let sum_mag = concat(a.clone(), b.clone()).mag();
            if sum_mag > max {
                max = sum_mag;
            }
        }
    }

    println!("{}", max);
}
