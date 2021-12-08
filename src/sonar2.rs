use std::io;

fn main() {
    let mut buffer = String::new();
    //let mut vals: Vec<isize> = Vec::new();
    let mut last_vals: [Option<usize>; 3] = [None, None, None];
    let mut increase_count = 0;
    let stdin = io::stdin();
    while let Ok(len) = stdin.read_line(&mut buffer) {
        if len == 0 {
            break;
        }
        let val = str::parse::<isize>(&buffer.trim()).unwrap();
        let new_vals: [Option<usize>; 3] = [last_vals[1], last_vals[2], Some(val as usize)];
        if last_vals.iter().all(|v| v.is_some()) {
            let old_sum: usize = last_vals.iter().map(|v| v.unwrap()).sum();
            let new_sum: usize = new_vals.iter().map(|v| v.unwrap()).sum();
            if new_sum > old_sum {
                increase_count += 1;
            }
        }
        last_vals = new_vals;

        //vals.push(val);
        buffer.clear();
    }
    println!("{}", increase_count);
}
