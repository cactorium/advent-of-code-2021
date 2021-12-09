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

// let's use the numbering scheme they have in the problem:
//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg
//

// using the 1 and 7 you can assume 3 of the segments are in one of two permutations
// c and f are still interchangeable but a is the one included in 7 but not
// in 1
// this leaves 4 segments that are entirely unconstrained,
// for a total of 2 * 1 * 4! = 48 permutations
// which is trivial to permute over in a depth first search
// so the solution is to calculate all these permutations
// and test each one to see if it solves the ten patterns

fn match_pattern(pattern: u8) -> Option<u8> {
    let (a, b, c, d, e, f, g) = (1u8, 2u8, 4u8, 8u8, 16u8, 32u8, 64u8);
    let numbers = vec![
        a | b | c | e | f | g,  // 0
        c | f,                  // 1
        a | c | d | e | g,      // 2
        a | c | d | f | g,      // 3
        b | c | d | f,          // 4
        a | b | d | f | g,      // 5
        a | b | d | e | f | g,  // 6
        a | c | f,              // 7
        a | b | c | d | e | f | g,      // 8
        a | b | c | d | f | g   // 9
    ];

    numbers
        .iter()
        .enumerate()
        .filter(|(_, n)| **n == pattern)
        .map(|(i, _)| i as u8)
        .next()
}

fn unpermute(pattern: u8, mapping: &[Option<u8>; 7]) -> u8 {
    mapping
        .iter()
        .enumerate()
        .fold(0u8, |a, (i, n)| {
            if (pattern & (1 << i)) != 0 { // if the pattern has an entry at this index
                a | (1 << n.unwrap()) // bitwise OR the equivalent bit in the mapped result
            } else {
                a
            }
        })
}

// the mapping is from the segment in the scrambled state back to the correct segment,
// mapping[scrambled] = real_segment
fn test_permutation(tens: &[u8; 10], mapping: &[Option<u8>; 7]) -> bool {
    // simple test; for each pattern in tens run it through the mapping and
    // make sure it matches one of the numbers
    //println!("testing {:?} {:?}", tens, mapping);
    tens.iter()
        .map(|v| unpermute(*v, mapping))
        .all(|v| match_pattern(v).is_some())
}

fn try_solve(tens: &[u8; 10], mapping: &[Option<u8>; 7], unsolved: u8) -> Option<[Option<u8>; 7]> {
    if unsolved == 0 {
        // permutation is finished, test it
        if test_permutation(tens, mapping) {
            Some(*mapping)
        } else {
            None
        }
    } else {
        // pick the next digit for the next unpicked entry in mapping
        let idx = mapping
            .iter()
            .enumerate()
            .filter(|(_, &c)| c.is_none())
            .map(|(i, _)| i)
            .next()
            .unwrap();
        let mut new_mapping = *mapping;
        // try setting the entry to any of the remaining segments
        for i in 0..7 {
            if (unsolved & (1 << i)) != 0 {
                new_mapping[idx] = Some(i);
                let new_unsolved = unsolved & !(1u8 << i);
                let maybe_solution = try_solve(tens, &new_mapping, new_unsolved);
                if maybe_solution.is_some() {
                    return maybe_solution;
                }
            }
        }
        return None;
    }
}

fn solve(tens: &[u8; 10]) -> [Option<u8>; 7] {
    // find the 1 and 7 cases
    let ones = tens
        .iter()
        .filter(|v| count_active(**v) == 2)
        .next()
        .map(|v| *v)
        .unwrap();
    let sevens = tens
        .iter()
        .filter(|v| count_active(**v) == 3)
        .next()
        .map(|v| *v)
        .unwrap();

    let a_shift = sevens & !ones;
    if count_active(a_shift) != 1 {
        panic!("invalid input; the seven entry does not share two segments with the one entry!");
    }
    let find_shift = |v: u8| -> usize {
        for i in 0..8 {
            if (v & (1 << i)) != 0 {
                return i as usize;
            }
        }
        return 8;
    };

    let a = find_shift(a_shift);
    let c = find_shift(ones); // c and f will be flipped around in the second set of permutations
    let f = find_shift(ones & !(1u8 << c)); // clear the first bit to get the second set bit
    
    // try the first set with a, c and f set like this:
    let mut maybe_mapping = [None; 7];
    maybe_mapping[a] = Some(0);
    maybe_mapping[c] = Some(2);
    maybe_mapping[f] = Some(5);
    let unsolved = 0x7f & !(1 << 0) & !(1 << 2) & !(1 << 5);
    //println!("start perm {:?}", &maybe_mapping);
    //println!("unsolved {:b}", unsolved);

    let maybe_solution = try_solve(tens, &maybe_mapping, unsolved);
    if maybe_solution.is_some() {
        return maybe_solution.unwrap();
    }

    maybe_mapping[f] = Some(2); // flip these two around
    maybe_mapping[c] = Some(5);
    // solve or bust motherfucker
    let maybe_solution = try_solve(tens, &maybe_mapping, unsolved);
    maybe_solution.unwrap()
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

    let mut sum = 0;
    for &(ref patterns, ref output) in &input {
        let mapping = solve(patterns);
        let value = output
            .iter()
            .map(|digit| match_pattern(unpermute(*digit, &mapping)).unwrap())
            .fold(0usize, |accum, digit| 10*accum + digit as usize);
        //println!("{:?} {}", mapping, value);
        println!("{}", value);
        sum += value;
    }
    println!("{}", sum);
}
