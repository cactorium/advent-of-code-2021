use std::cmp::Ordering;

use std::io::BufRead;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Path<'a, F> {
    cost: usize,
    pos: (usize, usize),
    f: &'a F
}

impl <'a, F> Path<'a, F> where F: Fn(usize, usize) -> usize {
    fn calc_cost(&self) -> usize {
        self.cost + (self.f)(self.pos.0, self.pos.1)
    }
}

impl <'a, F> PartialEq for Path<'a, F> where F: Fn(usize, usize) -> usize {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl <'a, F> Eq for Path<'a, F> where F: Fn(usize, usize) -> usize {
}


impl <'a, F> PartialOrd for Path<'a, F> where F: Fn(usize, usize) -> usize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // flipped so that the lowest score is best
        Some(other.calc_cost().cmp(&self.calc_cost()))
    }
}

impl <'a, F> Ord for Path<'a, F> where F: Fn(usize, usize) -> usize {
    fn cmp(&self, other: &Self) -> Ordering {
        // flipped so that the lowest score is best
        other.calc_cost().cmp(&self.calc_cost())
    }
}

fn astar(start: (usize, usize), end: (usize, usize), risk: &[Vec<usize>]) -> Option<(Vec<(usize, usize)>, usize)>{
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    let mut f_score: HashMap<(usize, usize), usize> = HashMap::new();

    let manchester = |(px, py): (usize, usize), (qx, qy): (usize, usize)| -> usize {
        let dx = ((px as isize) - (qx as isize)).abs();
        let dy = ((py as isize) - (qy as isize)).abs();
        (dx + dy) as usize
    };

    let calc_cost = |x: usize, y: usize| -> usize {
        manchester((x, y), end)
    };

    let mut paths = BinaryHeap::new();
    paths.push(Path {
        cost: 0,
        pos: start,
        f: &calc_cost,
    });
    g_score.insert(start, 0);
    f_score.insert(start, manchester(start, end));

    while paths.len() > 0 {
        let cur = paths.pop().unwrap();
        if cur.pos == end {
            let mut path = Vec::new();
            // TODO reconstruct path using came_from
            let mut cur_pos = cur.pos;
            path.push(cur_pos);

            while cur_pos != start {
                cur_pos = came_from[&cur_pos];
                path.push(cur_pos);
            }
            return Some((path.into_iter().rev().collect(), g_score[&cur.pos]));
        }

        // skip nodes where the cost is outdated
        if cur.cost > g_score[&cur.pos] {
            continue;
        }

        let (x, y) = cur.pos;

        // try to go in all four directions
        for (ox, oy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // bounds check
            let (nx, ny) = ((x as isize) + ox, (y as isize) + oy);
            if ny < 0 || ny >= (risk.len() as isize) {
                continue;
            }
            if nx < 0 || nx >= (risk[ny as usize].len() as isize) {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);
            // calculate the new score and see if it's the best
            let new_gscore = cur.cost + risk[ny][nx];
            let last_gscore = g_score.get(&(nx, ny));
            let is_better = match last_gscore {
                Some(val) => new_gscore < *val,
                None => true,
            };

            if is_better {
                g_score.insert((nx, ny), new_gscore);
                came_from.insert((nx, ny), cur.pos);
                // NOTE: this doesn't look for copies of the same point with a lower score,
                // so duplicates can exist
                // this is dealt with above where it checks to see if the cost of the
                // current node is outdated
                paths.push(Path {
                    cost: new_gscore,
                    pos: (nx, ny),
                    f: &calc_cost
                });
            }
        }
    }
    return None;
}

fn main() {
    let stdin = std::io::stdin();
    let small_risk: Vec<Vec<usize>> = stdin.lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.trim().len() != 0)
        .map(|l| l.trim().bytes()
                  .map(|b| (b - ('0' as u8)) as usize)
                  .collect())
        .collect();
    let risk_x_expanded: Vec<Vec<usize>> = 
        small_risk.iter()
            .map(|row|
                 (0..5)
                 .flat_map(|i| row
                           .iter()
                           .map(|v| ((8 + *v + i) % 9) + 1)
                           .collect::<Vec<_>>())
                 .collect())
            .collect();
    let mut risk: Vec<Vec<usize>> = Vec::new();
    for i in 0..5 {
        for row in &risk_x_expanded {
            risk.push(row
                      .iter()
                      .map(|v| ((8 + *v + i) % 9) + 1)
                      .collect());
        }
    }
    for row in &risk {
        println!("{:?}", &row);
    }

    println!("{:?}", astar((0, 0), (risk.len() - 1, risk[risk.len()-1].len() - 1), &risk));
}
