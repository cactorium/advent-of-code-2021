use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut polymer_str = String::new();
    stdin.read_line(&mut polymer_str).unwrap();
    polymer_str = polymer_str.trim().to_string();

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

    // naiively checking each pair results in a runtime of O(n),
    // we really need O(1) given how it grows exponentially
    // this is possible by using a vector/hashmap to represent the 
    // population of pairs, so that each step the entire population of each pair type
    // can be transformed using math rather than one pair at a time
    //
    // to do this we need to convert the polymer and rules into a usable format,
    // and at the end sum up the populations of all the pairs starting with
    // each possible character to get the final count
    
    // use an option for the second entry so that it's possible to represent the ending pair
    let mut polymer: HashMap<(u8, Option<u8>), usize> = HashMap::new();
    for (a, b) in polymer_str.bytes().zip(polymer_str.bytes()
                                          .skip(1)
                                          .map(|v| Some(v))
                                          .chain(Some(None))) {
        let entry = polymer.entry((a, b)).or_insert(0);
        *entry += 1;
    }
    let mut new_rules: HashMap<(u8, Option<u8>), u8> = HashMap::new();
    for ((a, b), o) in rules.iter() {
        new_rules.insert((*a, Some(*b)), *o);
    }
    // make immutable
    let new_rules = new_rules;

    println!("{:?}", &polymer);
    for _ in 0..40 {
        let mut new_polymer: HashMap<(u8, Option<u8>), usize> = HashMap::new();
        for ((a, maybe_b), count) in polymer.iter() {
            //so we're looking for a rule that matches AB
            match new_rules.get(&(*a, *maybe_b)) {
                Some(c) => {
                    // if there is one then it'll put a C in between A and B
                    // making AC and CB
                    let entry = new_polymer.entry((*a, Some(*c))).or_insert(0);
                    *entry += *count;
                    let entry2 = new_polymer.entry((*c, *maybe_b)).or_insert(0);
                    *entry2 += *count;
                },
                None => {
                    // else just copy the population of AB into the new
                    // hashmap
                    let entry = new_polymer.entry((*a, *maybe_b)).or_insert(0);
                    *entry += *count;
                }
            }
        }
        //println!("{:?}", &new_polymer);
        polymer = new_polymer;
    }

    let mut populations: HashMap<u8, usize> = HashMap::new();
    for ((a, _), count) in polymer.iter() {
        let entry = populations.entry(*a).or_insert(0);
        *entry += *count;
    }

    let max_count = *populations.values().max().unwrap();
    let min_count = *populations.values().min().unwrap();
    println!("{}", max_count - min_count);
}
