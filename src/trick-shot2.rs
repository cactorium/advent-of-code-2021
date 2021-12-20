fn calc_nsolutions(mi_x: isize, ma_x: isize, mi_y: isize, ma_y: isize) -> usize {
    // find the x velocities that let it reach inside [mi_x, ma_x]
    // the best velocity is where it reaches inside the target region
    // when the x velocity hits zero
    // (basically (x(x-1)/2) is just inside the region)
    // x velocity doesn't matter at this point, because this sets the minimum
    // amount of time the probe is allowed to have to reach its target, and
    // you want to maximize the time it has to give it as much y velocity as
    // possible

    // eh whatever let's just brute force it from a reasonable set of (x, y)
    // values
    let mut solutions = Vec::new();
    for start_vx in 0..(ma_x+1) {
        for start_vy in -std::cmp::max(mi_y.abs(), ma_y.abs())..std::cmp::max(mi_y.abs(), ma_y.abs()) {
            let mut vx = start_vx;
            let mut vy = start_vy;
            let (mut x, mut y) = (0, 0);
            while ((vy >= 0) || (y >= mi_y)) && (((x >= mi_x) && (x <= ma_x)) || (vx != 0)) {
                x += vx;
                y += vy;


                if (x >= mi_x) && (x <= ma_x) && (y >= mi_y) && (y <= ma_y) {
                    println!("found solution ({}, {})", start_vx, start_vy);
                    solutions.push((start_vx, start_vy));
                    break;
                }

                if vx > 0 {
                    vx -= 1;
                }
                vy -= 1;
            }
        }
    }
    println!("{:?}", &solutions);
    solutions.len()
}

fn main() {
    let (min_x, max_x): (isize, isize) = (282, 314);
    let (min_y, max_y): (isize, isize) = (-80, -45);
    /*
    let (min_x, max_x): (isize, isize) = (20, 30);
    let (min_y, max_y): (isize, isize) = (-10, -5);
    */

    let solution = calc_nsolutions(min_x, max_x, min_y, max_y);
    println!("{}", solution);
}
