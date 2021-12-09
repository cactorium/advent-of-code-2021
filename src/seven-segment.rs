fn alpha2u8(s: &str) -> u8 {
    "abcdefg".bytes()
        .enumerate()
        .filter(|(_, c)| s.bytes().any(|d| d == *c))
        .map(|(i, _)| (1 << i) as u8)
        .fold(0, |a, b| a | b)
}

fn parse_line(s: &str) -> Option<([u8; 10], [u8; 4])> {
    let halves: Vec<_> = s.trim().split(" | ").collect();
    if halves.len() != 2 {
        println!("could not split line in half: {}", s);
        return None;
    }
    let ten_patterns: Vec<_> = halves[0].split(" ").map(alpha2u8).collect();
    if ten_patterns.len() != 10 {
        println!("could not split first half into 10: {}", s);
        return None;
    }
    let mut ten_out = [0u8; 10];
    for i in 0..10 {
        ten_out[i] = ten_patterns[i];
    }
    let output_patterns: Vec<_> = halves[1].split(" ").map(alpha2u8).collect();
    if output_patterns.len() != 4 {
        println!("could not split second half into 4: {}", s);
        return None;
    }
    let mut output = [0u8; 4];
    for i in 0..4 {
        output[i] = output_patterns[i];
    }
    Some((ten_out, output))
}

fn count_active(v: u8) -> usize {
    (0..8).filter(|shift| (v & (1<<shift)) != 0).count()
}

fn main() {
    let stdin = std::io::stdin();

    let mut input: Vec<([u8; 10], [u8; 4])> = Vec::new();
    let mut buffer = String::new();

    while let Ok(len) = stdin.read_line(&mut buffer) {
        if len == 0 {
            break;
        }
        input.push(parse_line(buffer.as_str()).unwrap());
        buffer.clear();
    }

    let mut active_count = 0;
    for &(ref _, ref output) in &input {
        active_count += output
            .iter()
            .map(|v| count_active(*v))
            .filter(|v| (*v == 2) || (*v == 3) || (*v == 4) || (*v == 7))
            .count();
        println!("{}", active_count);
    }
}
