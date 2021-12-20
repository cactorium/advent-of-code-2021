use std::io::BufRead;
use std::ops::{Add, Sub};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct V(isize, isize, isize);

impl Add<V> for V {
    type Output = V;
    fn add(self, rhs: V) -> V {
        V(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<V> for V {
    type Output = V;
    fn sub(self, rhs: V) -> V {
        V(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl V {
    fn rotate_z(&self) -> V {
        V(-self.1, self.0, self.2)
    }
    fn rotate_y(&self) -> V {
        V(self.2, self.1, -self.0)
    }
    fn rotate_x(&self) -> V {
        V(self.0, -self.2, self.1)
    }

    fn sq_mag(&self) -> u64 {
        let (dx, dy, dz) = (self.0.abs() as u64,
            self.1.abs() as u64, self.2.abs() as u64);
        dx*dx + dy*dy + dz*dz
    }

    fn manhattan(&self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

// representation of 90 degree rotations, ordered z-y-x
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct R(usize, usize, usize);

impl R {
    fn transform(&self, mut v: V) -> V {
        for _ in 0..self.0 {
            v = v.rotate_z();
        }
        for _ in 0..self.1 {
            v = v.rotate_y();
        }
        for _ in 0..self.2 {
            v = v.rotate_x();
        }
        v
    }

    fn inv(&self, mut v: V) -> V {
        for _ in 0..(4-self.2) {
            v = v.rotate_x();
        }
        for _ in 0..(4-self.1) {
            v = v.rotate_y();
        }
        for _ in 0..(4-self.0) {
            v = v.rotate_z();
        }
        v
    }
}

#[derive(Debug, Clone, Copy)]
struct Transform {
    offset: V,
    rotation: R,
}

impl Transform {
    fn transform(&self, v: V) -> V {
        self.offset + self.rotation.transform(v)
    }
    fn inv(&self, v: V) -> V {
        self.rotation.inv(v - self.offset)
    }

}

fn main() {
    let stdin = std::io::stdin();

    let mut coords: Vec<Vec<V>> = Vec::new();
    let mut cur_scanner: Vec<V> = Vec::new();

    let mut first_line = true;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.starts_with("---") {
            if !first_line {
                coords.push(cur_scanner.clone());
                cur_scanner.clear();
            }
            first_line = false;
            continue;
        }
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let nums: Vec<isize> = line
            .split(",")
            .map(|s| str::parse::<isize>(s).unwrap())
            .collect();
        if nums.len() != 3 {
            panic!("invalid line {}", line);
        }
        cur_scanner.push(V(nums[0], nums[1], nums[2]));
    }
    
    coords.push(cur_scanner.clone());

    // so to align the scanners we're going to generate all the triangles
    // that can be formed from combinations of the coordinates each scanner
    // collected, and use the sorted distances between the points
    // to provide an orientation invariant way to represent the triangles
    // and store that in a HashMap for each scanner, with indices
    // to the points used stored in there
    //
    // TODO maybe store ambiguous distances somewhere to skip them in the comparison later,
    // there should be enough triangles to work even if we lose some
    let mut triangles: Vec<HashMap<(u64, u64, u64), (usize, usize, usize)>> = Vec::new();
    for (scanner_num, scanner) in coords.iter().enumerate() {
        let mut tris: HashMap<(u64, u64, u64), (usize, usize, usize)> = HashMap::new();
        for (i, p) in scanner.iter().enumerate() {
            for (j, q) in scanner.iter().enumerate().skip(i + 1) {
                for (k, r) in scanner.iter().enumerate().skip(j + 1) {
                    let pq = (*p - *q).sq_mag();
                    let pr = (*p - *r).sq_mag();
                    let qr = (*q - *r).sq_mag();

                    // distances are stored shortest, medium. longest
                    // points are stored (one adjacent to)
                    // shortest-longest, shortest-medium, medium-longest
                    // if distances are equal, this shouldn't be used
                    // to figure out orientation because
                    // of the resulting ambiguity
                    // there's six cases so I'm just going to write
                    // it out by hand
                    // i = p, j = q, k = r
                    let (a, b, c, d, e, f) = 
                        if pq < pr {
                            if pr < qr {
                                // pq < pr < qr
                                // q, p, r
                                (pq, pr, qr, j, i, k)
                            } else {
                                if pq < qr {
                                    // pq < qr < pr
                                    // p, q, r
                                    (pq, qr, pr, i, j, k)
                                } else {
                                    // pq < pr, pr >= qr, pq >= qr
                                    // qr <= pq < pr
                                    // r, q, p
                                    (qr, pq, pr, k, j, i)
                                }
                            }
                        } else {
                            // pr <= pq
                            if pq < qr {
                                // pr <= pq < qr
                                // r, p, q
                                (pr, pq, qr, k, i, j)
                            } else {
                                // pr <= pq, qr <= pq
                                if pr < qr {
                                    // pr < qr <= pq
                                    // p, r, q
                                    (pr, qr, pq, i, k, j)
                                } else {
                                    // qr <= pr <= pq
                                    // q, r, p
                                    (qr, pr, pq, j, k, i)
                                }
                            }
                        };
                    if tris.contains_key(&(a, b, c)) {
                        println!("W: ambiguous triangle {} {} {} {}", scanner_num, i, j, k);
                    }
                    tris.insert((a, b, c), (d, e, f));
                }
            }
        }
        triangles.push(tris);
    }

    // then we'll compare these HashSets between scanners, and whenever
    // 12 points match up we'll compute the orientation transformation between
    // the two (rotation plus offsets in x, y, and z)

    let mut transforms: Vec<Vec<Option<Transform>>> = Vec::new();
    for _ in 0..triangles.len() {
        transforms.push(vec![None; triangles.len()]);
    }

    for (i, a) in triangles.iter().enumerate() {
        for (j, b) in triangles.iter().enumerate().skip(i + 1) {
            let mut matching_a: HashSet<usize> = HashSet::new();
            let mut matching_b: HashSet<usize> = HashSet::new();
            let mut matching_dists: Vec<(u64, u64, u64)> = Vec::new();
            for (dists, (y, u, v)) in a.iter() {
                if b.contains_key(dists) {
                    matching_dists.push(*dists);
                    matching_a.insert(*y);
                    matching_a.insert(*u);
                    matching_a.insert(*v);
                    let (q, r, s) = b[dists];
                    matching_b.insert(q);
                    matching_b.insert(r);
                    matching_b.insert(s);
                }
            }
            if matching_a.len() >= 12 {
                println!("scanner {} overlaps {}", i, j);
                // TODO try more triangles in case the first one fails

                // let's calculate the rotation by seeing how the points are
                // rotated relative to the first point in each triangle
                //
                // this works even with an offset because linearity
                // around the rotation matrices or whatever
                // retry if the current point doesn't work
                let usable_dists = matching_dists.iter()
                    .filter(|&(a, b, c)| *a != *b && *b != *c)
                    .next();
                if usable_dists.is_none() {
                    panic!("couldn't find usable dists for {} {}", i, j);
                }
                let dists = usable_dists.unwrap();
                let (a0, a1, a2) = a[dists];
                let (b0, b1, b2) = b[dists];

                let av = coords[i][a1] - coords[i][a0];
                let bv = coords[j][b1] - coords[j][b0];

                //println!("aligning coords {:?} <= {:?}", &av, &bv);

                // try all possible rotations and see if any of them
                // work
                let try_rotate_x = |mut v: V| -> Option<usize> {
                    for turns_x in 0..4 {
                        //println!("trying {:?} => {:?}", &v, &av);
                        if v == av {
                            return Some(turns_x)
                        }
                        v = v.rotate_x();
                    }
                    None
                };
                let try_rotate_y = |mut v: V| -> Option<(usize, usize)> {
                    for turns_y in 0..4 {
                        if let Some(turns_x) = try_rotate_x(v) {
                            return Some((turns_y, turns_x));
                        }
                        v = v.rotate_y();
                    }
                    None
                };
                let try_rotate_z = |mut v: V| -> Option<R> {
                    for turns_z in 0..4 {
                        if let Some((turns_y, turns_x)) = try_rotate_y(v) {
                            return Some(R(turns_z, turns_y, turns_x));
                        }
                        v = v.rotate_z();
                    }
                    None
                };

                match try_rotate_z(bv) {
                    Some(r) => {
                        println!("found rotation {:?}", r);
                        // so now we have a rotation that takes an offset version of the
                        // coordinates in `b and maps it into an offset version of the coordinates
                        // in `a
                        //
                        // a - offseta = T(b - offsetb)
                        //
                        // we want the formula that converts a coordinate in b directly
                        // into a coordinate in a:
                        //
                        // a = f(b)
                        //
                        // which we can solve for using the above known expression:
                        //
                        // a = T(b) - T(offsetb) + offseta
                        //
                        // so we basically need to repeat the rotation on the `b offset
                        // (the first point of the triangle we subtracted from
                        // earlier) and add the `a offset (the equivalent point
                        // in `a) to get the overall transformation from b to a
                        let offset = coords[i][a0] - r.transform(coords[j][b0]);
                        println!("calc offset {:?}", &offset);
                        // check against all the third point to verify
                        let a2 = coords[i][a2];
                        let b2_t = r.transform(coords[j][b2]) + offset;

                        if a2 == b2_t {
                            println!("transform verified");
                            transforms[i][j] = Some(Transform {
                                rotation: r,
                                offset: offset,
                            });
                        } else {
                            println!("transform failed, {:?} != {:?}", &a2, &b2_t);
                        }
                    },
                    None => {
                        unreachable!("unable to realign triangles")
                    }
                }
            }
        }
    }
    // using this we should end up with a set of invertible transformations
    // from any scanner frame to any other one
    // which we'll use to transform all the beacon coordinates to being relative
    // to scanner 0, and then use a HashSet to iterate through all the scanners'
    // beacons to find all the unique beacons found

    // store either the transform to do the operation or its inverse
    #[derive(Debug, Clone, Copy)]
    enum T {
        T(Transform),
        I(Transform),
    }


    fn find_path<'a> (transforms: &Vec<Vec<Option<Transform>>>, cur_coord: usize, path: &mut Vec<(usize, usize, T)>, nodes_touched: &mut HashSet<usize>) -> Option<Vec<(usize, usize, T)>> {
        if cur_coord == 0 {
            return Some(path.clone());
        }
        // check both the row and column that cur_coord cooresponds to for usable transforms
        for i in 0..transforms.len() {
            if nodes_touched.contains(&i) {
                continue;
            }

            match &transforms[i][cur_coord] {
                &Some(t) => {
                    nodes_touched.insert(i);
                    path.push((cur_coord, i, T::T(t)));
                    if let Some(ret) = find_path(transforms, i, path, nodes_touched) {
                        return Some(ret);
                    }
                    path.pop();
                    nodes_touched.remove(&i);
                },
                &None => {}
            }
        }
        for j in 0..transforms[cur_coord].len() {
            if nodes_touched.contains(&j) {
                continue;
            }
            match &transforms[cur_coord][j] {
                &Some(t) => {
                    nodes_touched.insert(j);
                    path.push((cur_coord, j, T::I(t)));
                    if let Some(ret) = find_path(transforms, j, path, nodes_touched) {
                        return Some(ret);
                    }
                    path.pop();
                    nodes_touched.remove(&j);
                },
                &None => {}
            }
        }

        None
    }

    let mut nodes = HashSet::new();
    let mut path = Vec::new();

    let mut transform_paths: Vec<Vec<T>> = Vec::new();
    for i in 0..transforms.len() {
        nodes.clear();
        path.clear();

        nodes.insert(i);
        let path = find_path(&transforms, i, &mut path, &mut nodes).unwrap();
        //println!("{:?}", path.iter().map(|(a, b, _)| (*a, *b)).collect::<Vec<_>>());
        transform_paths.push(path
                             .iter()
                             .map(|(_, _, t)| *t)
                             .collect());
    }
    //println!("{:?}", &transform_paths);
    
    let mut points: Vec<V> = Vec::new();
    // calculate where the origin is for each scanner
    for (i, tp) in transform_paths.iter().enumerate() {
        let mut pt = V(0, 0, 0);
        for t in tp.iter() {
            pt = match *t {
                T::T(tx) => tx.transform(pt),
                T::I(tx) => tx.inv(pt),
            };
        }
        points.push(pt);
    }
    println!("{:?}", &points);

    let mut max_dist = 0;
    for (i, p) in points.iter().enumerate() {
        for q in points.iter().skip(i + 1) {
            let dist = (*p - *q).manhattan();
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }
    println!("{}", max_dist);
}
