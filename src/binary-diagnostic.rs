fn count_string(count: &mut Vec<(usize, usize)>, s: &str) {
    if count.len() < s.len() {
        count.resize(s.len(), (0, 0));
    }

    for (c, &mut (ref mut zero_count, ref mut one_count)) in s.bytes().
        zip(count.iter_mut()) {
        if c == '0' as u8 {
            *zero_count += 1;
        } else if c == '1' as u8 {
            *one_count += 1;
        } else {
            panic!("invalid character {}", s);
        }
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut buffer = String::new();

    let mut count: Vec<(usize, usize)> = Vec::new();

    while let Ok(len) = stdin.read_line(&mut buffer) {
        if len == 0 {
            break;
        }

        let trimmed = buffer.trim();
        count_string(&mut count, &trimmed);
        buffer.clear();
    }

    let epsilon: String = count.iter().map(|&(ref zero, ref one)| {
        if *one > *zero {
            '1'
        } else {
            '0'
        }
    }).collect();
    let gamma: String = count.iter().map(|&(ref zero, ref one)| {
        if *one > *zero {
            '0'
        } else {
            '1'
        }
    }).collect();

    let str2int = |s: &String| -> u64 {
        s.chars().fold(0, |x, c| {
            match c {
                '0' => x << 1,
                '1' => (x << 1) + 1,
                _ => unreachable!("invalid epsilon")
            }
        })
    };
    let epsilon_val = str2int(&epsilon);
    let gamma_val = str2int(&gamma);

    println!("{} {}", epsilon, gamma);
    println!("{} {}", epsilon_val, gamma_val);
    println!("{}", epsilon_val*gamma_val);
}
