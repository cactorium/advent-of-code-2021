use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut polymer = String::new();
    stdin.read_line(&mut polymer).unwrap();
    polymer = polymer.trim().to_string();

    let mut tmp = String::new();
    stdin.read_line(&mut tmp).unwrap();

    let mut rules: HashMap<(u8, u8), u8> = HashMap::new();
    for lne in stdin.lock().lines() {
        let line = lne.unwrap().trim().to_string();
        if line.len() == 0 {
            break;
        }
        let parts: Vec<_> = line.split(" -> ").collect();
        let match_path = parts[0].as_bytes();
        if match_path.len() != 2 {
            panic!("invalid match {}", &line);
        }
        let output = parts[1].as_bytes();
        if output.len() != 1 {
            panic!("invalid output {}", &line);
        }
        rules.insert((match_path[0], match_path[1]), output[0]);
    }

    println!("{:?}", &rules);
    println!("{}", &polymer);
    for i in 0..10 {
        let polymer_bytes: Vec<u8> = polymer.bytes()
            .zip(polymer.bytes()
                 .skip(1)
                 .map(|v| Some(v))
                 .chain(Some(None))
                 )
            .flat_map(|(a, b)| match (a, b) {
                (a, Some(b)) => match rules.get(&(a, b)) {
                    Some(o) => vec![a, *o],
                    None => vec![a],
                },
                (a, None) => vec![a],
            })
            .collect();
        polymer = String::from_utf8(polymer_bytes).unwrap();
        println!("step {}: {}", i, &polymer);
    }
    let mut population: HashMap<u8, usize> = HashMap::new();
    for b in polymer.bytes() {
        let count = population.entry(b).or_insert(0);
        *count += 1;
    }
    let max_count = *population.values().max().unwrap();
    let min_count = *population.values().min().unwrap();
    println!("{}", max_count - min_count);
}
