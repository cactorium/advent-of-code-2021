use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut mapping_str = String::new();

    stdin.read_line(&mut mapping_str).unwrap();

    let mapping: Vec<_> = mapping_str.trim()
        .bytes()
        .map(|b| b == ('#' as u8))
        .collect();

    stdin.read_line(&mut mapping_str).unwrap();

    let input: Vec<Vec<bool>> = stdin.lock().lines()
        .map(|l| {
            let ln = l.unwrap();
            let buf = ln.trim();
            buf
                .bytes()
                .map(|b| b == ('#' as u8))
                .collect::<Vec<bool>>()
        })
        .collect();

    println!("{:?}", &input);

    let gen_image = |image: &Vec<Vec<bool>>, pass: usize| -> Vec<Vec<bool>> {
         let mut oob_idx = 0;
         if pass >= 1 {
             for _ in 0..(pass - 1) {
                 oob_idx = if mapping[oob_idx] {
                     0x1ff
                 } else {
                     0
                 };
             }
         }
         println!("idx: {} {}", oob_idx, mapping[oob_idx]);

        (0..(image.len() + 12))
        .map(|j| (0..(image[0].len() + 12))
             .map(|i| {
                 let y = (j as isize) - 5;
                 let x = (i as isize) - 5;
                 (0..9)
                     .map(|i| (i % 3, i / 3))
                     .map(|(ox, oy)| (y + oy, x + ox))
                     .map(|(py, px)| {
                         //print!("({} {}) ", px, py);
                         if py < 0 || py >= (image.len() as isize) ||
                             px < 0 || px >= (image[0].len() as isize) {
                             if pass == 0 {
                                 0
                             } else {
                                 if mapping[oob_idx] {
                                     1
                                 } else {
                                     0
                                 }
                             }
                         } else {
                             if image[py as usize][px as usize] {
                                 1
                             } else {
                                 0
                             }
                         }
                     })
                    .fold(0, |acc, v| (acc << 1) | v)
             })
             .map(|idx| {
                 //println!("{}", idx);
                 mapping[idx]
             })
             .collect::<Vec<_>>()
         )
        .collect::<Vec<_>>()
    };

    let intermediate = gen_image(&input, 0);
    for row in intermediate.iter() {
        for p in row.iter() {
            if *p {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
    println!("");

    let output_image = gen_image(&intermediate, 1);

    for row in output_image.iter() {
        for p in row.iter() {
            if *p {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    println!("{}", output_image.iter()
             .map(|l| l.iter().filter(|p| **p).count())
             .sum::<usize>());
}
