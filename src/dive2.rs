use std::io;

#[derive(Clone, Copy, Debug)]
enum Command {
    Up(isize),
    Down(isize),
    Fwd(isize)
}

fn main() {
    let mut buffer = String::new();
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut aim: isize = 0;

    let stdin = io::stdin();

    let mut cmds = Vec::new();

    while let Ok(len) = stdin.read_line(&mut buffer) {
        if len == 0 {
            break;
        }
        let parts: Vec<_> = buffer.split(' ').collect();
        if parts.len() != 2 {
            println!("invalid line {}", buffer);
        }
        let dist: isize = str::parse(parts[1].trim()).unwrap();
        let cmd = match parts[0] {
            "forward" => Command::Fwd(dist),
            "up" => Command::Up(dist),
            "down" => Command::Down(dist),
            _ => {
                println!("invalid command {}", buffer);
                return;
            }
        };
        cmds.push(cmd);

        buffer.clear();
    }

    for cmd in &cmds {
        match cmd {
            &Command::Fwd(v) => { x += v; y += aim*v; },
            &Command::Up(v) => aim -= v,
            &Command::Down(v) => aim += v,
        }
    }

    println!("{}", x*y);
}
