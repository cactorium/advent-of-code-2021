use std::io;

fn main() {
    let mut buffer = String::new();
    //let mut vals: Vec<isize> = Vec::new();
    let mut last_val = None;
    let mut increase_count = 0;
    let stdin = io::stdin();
    while let Ok(len) = stdin.read_line(&mut buffer) {
        if len == 0 {
            break;
        }
        let val = str::parse::<isize>(&buffer.trim()).unwrap();
        if last_val.is_some() && last_val.unwrap() < val {
            increase_count += 1;
        }
        last_val = Some(val);

        //vals.push(val);
        buffer.clear();
    }
    println!("{}", increase_count);
}
