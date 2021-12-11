use std::collections::HashSet;

fn main() {
    let stdin = std::io::stdin();

    let mut octopus: Vec<Vec<u8>> = Vec::new();
    let mut buffer = String::new();
    for _ in 0..10 { 
        stdin.read_line(&mut buffer).unwrap();
        let row: Vec<u8> = buffer.trim().bytes()
            .map(|b| b - ('0' as u8))
            .collect();
        if row.len() != 10 {
            panic!("invalid line {}", &buffer);
        }
        octopus.push(row);

        buffer.clear();
    }

    let mut toflash: Vec<(usize, usize)> = Vec::new();
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut flash_count = 0;

    for step in 1..101 {
        flashed.clear();

        for i in 0..10 {
            for j in 0..10 {
                octopus[i][j] += 1;
                if !flashed.contains(&(i, j)) && octopus[i][j] > 9 {
                    toflash.push((i, j));
                }
            }
        }
        while let Some((fi, fj)) = toflash.pop() {
            if flashed.contains(&(fi, fj)) {
                continue;
            }
            println!("flash {} {}", fi, fj);
            flashed.insert((fi, fj));
            // increment its eight neighbors and see if any of them are flashing
            for (oi, oj) in [
                            (-1, -1), (-1, 0), (-1, 1),
                            (0, -1), (0, 1),
                            (1, -1), (1, 0), (1, 1)] {
                let (ni, nj) = ((fi as isize) + oi, (fj as isize) + oj);
                if ni < 0 || ni >= 10 || nj < 0 || nj >= 10 {
                    continue;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                octopus[ni][nj] += 1;
                if !flashed.contains(&(ni, nj)) && octopus[ni][nj] > 9 {
                    toflash.push((ni, nj));
                }
            }
        }
        flash_count += flashed.len();
        for (i, j) in &flashed {
            octopus[*i][*j] = 0;
        }
        flashed.clear();
        println!("step {}", step);
        for row in &octopus {
            println!("{}", row.iter().map(|v| (v + ('0' as u8)) as char).collect::<String>());
        }
    }
    println!("{}", flash_count);
}
