use std::io::BufRead;
use std::collections::HashSet;

fn find_basin(basin: &[Vec<u8>], point: (usize, usize), in_basin: &mut HashSet<(usize, usize)>) {
    let (i, j) = point;

    // terminating case
    if basin[i][j] == 9 {
        return;
    }
    // if it's already in there we don't need to check again
    if in_basin.contains(&(i, j)) {
        return;
    }

    // else add it and check the neighboring squares
    in_basin.insert((i, j));
    if i > 0 {
        find_basin(basin, (i - 1, j), in_basin);
    }
    if i < basin.len() - 1 {
        find_basin(basin, (i + 1, j), in_basin);
    }
    if j > 0 {
        find_basin(basin, (i, j - 1), in_basin);
    }
    if j < basin[i].len() - 1 {
        find_basin(basin, (i, j + 1), in_basin);
    }
}

fn main() {
    let stdin = std::io::stdin();
    let basin: Vec<Vec<u8>> = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().trim().to_string())
        .filter(|s| s.len() != 0)
        .map(|s| s.bytes().map(|c| c - ('0' as u8)).collect())
        .collect();

    println!("{:?}", &basin);

    let mut low_points: Vec<(usize, usize)> = Vec::new();
    let mut basins: Vec<usize> = Vec::new();
    for (i, line) in basin.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            let mut lowest = true;
            for (ox, oy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x = ox + (j as isize);
                let y = oy + (i as isize);
                if x >= 0 && x < (line.len() as isize) &&
                    y >= 0 && y < (basin.len() as isize) {
                    if basin[y as usize][x as usize] <= *val {
                        lowest = false;
                        break;
                    }
                }
            }
            if lowest {
                low_points.push((i, j));
                let mut basin_points = HashSet::new();
                find_basin(&basin, (i, j), &mut basin_points);
                basins.push(basin_points.iter().count());
            }
        }
    }
    basins.sort();
    println!("{:?}", basins);
    println!("{}", basins[basins.len() - 1]*basins[basins.len() - 2]*basins[basins.len() - 3]);
}
