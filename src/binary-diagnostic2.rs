fn main() {
    let stdin = std::io::stdin();

    let mut buffer = String::new();
    let mut strings: Vec<Vec<u8>> = Vec::new();

    while let Ok(len) = stdin.read_line(&mut buffer) {
        if len == 0 {
            break;
        }

        let trimmed = buffer.trim();
        strings.push(trimmed.chars()
                     .map(|c| match c {
                         '0' => 0,
                         '1' => 1,
                         _ => panic!("invalid input: {}", &trimmed)
                     })
                     .collect());
        buffer.clear();
    }

    let oxygen_generator: Vec<u8> = {
        let mut strings2 = strings.clone();
        let string_len = strings2[0].len();
        for i in 0..string_len {
            if strings2.len() == 1 {
                break;
            }
            let ones: Vec<_> = strings2.iter()
                .filter(|s| s[i] == 1)
                .collect();
            let zeros: Vec<_> = strings2.iter()
                .filter(|s| s[i] == 0)
                .collect();
            if ones.len() >= zeros.len() {
                strings2 = ones.iter().map(|s| (*s).clone()).collect();
            } else {
                strings2 = zeros.iter().map(|s| (*s).clone()).collect();
            }
        }
        strings2[0].clone()
    };

    let co2_scrubber = {
        let mut strings2 = strings.clone();
        let string_len = strings2[0].len();
        for i in 0..string_len {
            if strings2.len() == 1 {
                break;
            }
            let ones: Vec<&Vec<u8>> = strings2.iter()
                .filter(|s| s[i] == 1)
                .collect();
            let zeros: Vec<&Vec<u8>> = strings2.iter()
                .filter(|s| s[i] == 0)
                .collect();
            if zeros.len() <= ones.len() {
                strings2 = zeros.iter().map(|s| (*s).clone()).collect();
            } else {
                strings2 = ones.iter().map(|s| (*s).clone()).collect();
            }
        }
        strings2[0].clone()
    };
    println!("{:?} {:?}", &oxygen_generator, &co2_scrubber);
    let oxygen_rating = oxygen_generator
        .iter()
        .fold(0, |accum, v| (accum << 1) + (*v as usize));
    let co2_rating = co2_scrubber
        .iter()
        .fold(0, |accum, v| (accum << 1) + (*v as usize));
    println!("{} {} {}", oxygen_rating, co2_rating, oxygen_rating*co2_rating);
}
