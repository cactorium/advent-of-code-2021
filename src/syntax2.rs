use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().trim().to_string())
        .collect();

    let mut scores = Vec::new();
    for l in lines {
        let mut symbols: Vec<char> = Vec::new();
        let mut errored = false;
        for c in l.chars() {
            match c {
                '(' | '[' | '{' | '<' => symbols.push(c),
                ')' | ']' | '}' | '>' => {
                    let (matching, _points) = match c {
                        ')' => ('(', 3),
                        ']' => ('[', 57),
                        '}' => ('{', 1197),
                        '>' => ('<', 25137),
                        _ => unreachable!("shouldn't have come here")
                    };
                    if let Some(symbol) = symbols.pop() {
                        if matching != symbol {
                            errored = true;
                            break;
                        }
                    } else {
                        errored = true;
                        break;
                    }
                },
                _ => {
                    panic!("invalid character on {}", l);
                }
            }
        }
        // good line
        if !errored {
            println!("complete line {}, toautofill = {:?}", l, &symbols);
            let points = symbols.iter().rev()
                .fold(0usize, |acc, c| match *c {
                    '(' => 5*acc + 1,
                    '[' => 5*acc + 2,
                    '{' => 5*acc + 3,
                    '<' => 5*acc + 4,
                    _ => unreachable!("invalid character reached")
                });
            println!("points = {}", points);
            scores.push(points);
        }
    }
    println!("{:?}", scores);
    scores.sort();
    println!("{:?}", scores[scores.len()/2]);

}
