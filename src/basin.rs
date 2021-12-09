use std::io::BufRead;

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

    let mut low_sum: usize = 0;
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
                println!("lowest @{},{} {} {}", i, j, *val, low_sum);
                low_sum += (*val as usize) + 1;
            }
        }
    }
    println!("{}", low_sum);
}
