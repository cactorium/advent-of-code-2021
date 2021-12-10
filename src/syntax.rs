use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().trim().to_string())
        .collect();

    let mut errors = 0;
    for l in lines {
        let mut symbols: Vec<char> = Vec::new();
        for c in l.chars() {
            match c {
                '(' | '[' | '{' | '<' => symbols.push(c),
                ')' | ']' | '}' | '>' => {
                    let (matching, points) = match c {
                        ')' => ('(', 3),
                        ']' => ('[', 57),
                        '}' => ('{', 1197),
                        '>' => ('<', 25137),
                        _ => unreachable!("shouldn't have come here")
                    };
                    if let Some(symbol) = symbols.pop() {
                        if matching != symbol {
                            errors += points;
                            break;
                        }
                    } else {
                        println!("ran out of characters {}", l);
                        break;
                    }
                },
                _ => {
                    panic!("invalid character on {}", l);
                }
            }
        }
    }
    println!("{}", errors);
}
