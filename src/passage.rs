use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

fn isupper(s: &String) -> bool {
    s.chars().all(
        |c| ((c as u8) >= ('A' as u8)) && ((c as u8) <= ('Z' as u8))
        )
}

fn count_paths<'a>(segments: &'a HashMap<String, Vec<String>>, cur_path: Vec<&'a String>, passed: &mut HashSet<&'a String>) -> usize {
    if cur_path.len() > 0 && cur_path[cur_path.len()-1] == "end" {
        println!("{:?}", &cur_path);
        return 1;
    }

    let cur_cave = cur_path[cur_path.len() - 1].to_string();
    let mut path_count = 0;
    for possible_cave in segments[&cur_cave].iter() {
        // if it's an upper case cave or we haven't passed it yet
        if isupper(possible_cave) || !passed.contains(possible_cave) {
            // explore it
            let already_has_cave = passed.contains(possible_cave);
            passed.insert(possible_cave);

            let mut new_path = cur_path.clone();
            new_path.push(possible_cave);

            path_count += count_paths(segments, new_path, passed);

            // reset the passed hashset by removing the cave if it didn't
            // already have it
            if !already_has_cave {
                passed.remove(possible_cave);
            }
        }
    }
    path_count
}

fn main() {
    let stdin = std::io::stdin();
    let mut segments: HashMap<String, Vec<String>> = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.trim().split("-").collect();
        let first = segments.entry(parts[0].to_string()).or_insert(Vec::new());
        first.push(parts[1].to_string());
        let second = segments.entry(parts[1].to_string()).or_insert(Vec::new());
        second.push(parts[0].to_string());
    }
   
    let start = "start".to_string();
    let mut passed = HashSet::new();
    passed.insert(&start);
    println!("{}", count_paths(&segments, vec![&start], &mut passed));
}
