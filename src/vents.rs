use std::cmp;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_line(s: &String) -> Option<Line> {
    let trimmed = s.trim();
    let split: Vec<_> = trimmed.split(" -> ").collect();
    let start_parts: Vec<_> = split[0].split(",").collect();
    let end_parts: Vec<_> = split[1].split(",").collect();

    let start_x = str::parse::<usize>(start_parts[0]).ok()?;
    let start_y = str::parse::<usize>(start_parts[1]).ok()?;
    let end_x = str::parse::<usize>(end_parts[0]).ok()?;
    let end_y = str::parse::<usize>(end_parts[1]).ok()?;

    Some(Line {
        start: (start_x, start_y),
        end: (end_x, end_y),
    })
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = Vec::new();
    let mut buffer = String::new();
    while let Ok(len) = stdin.read_line(&mut buffer) {
        if len == 0 {
            break;
        }
        lines.push(parse_line(&buffer).unwrap());
        buffer.clear();
    }

    println!("{:?}", &lines);

    let mut overlaps: HashMap<(usize, usize), usize> = HashMap::new();
    for l in &lines {
        let (sx, sy) = l.start;
        let (ex, ey) = l.end;
        if sx == ex {
            let start_y = cmp::min(sy, ey);
            let end_y = cmp::max(sy, ey);
            for i in start_y..(end_y+1) {
                let entry = overlaps.entry((sx, i)).or_insert(0);
                *entry += 1;
            }
        }
        if sy == ey {
            let start_x = cmp::min(sx, ex);
            let end_x = cmp::max(sx, ex);
            for i in start_x..(end_x+1) {
                let entry = overlaps.entry((i, sy)).or_insert(0);
                *entry += 1;
            }
        }
    }

    println!("{}", overlaps.values().filter(|&v| *v >= 2).count());
}
