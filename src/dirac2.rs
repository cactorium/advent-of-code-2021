use std::collections::HashMap;

fn main() {
    // store the current list of all universes as a 4d hashmap of each player's scores and
    // positions
    let mut universes: HashMap<(usize, usize, usize, usize), u64> = HashMap::new();
    universes.insert((6, 0, 1, 0), 1);

    let mut one_won: u64 = 0;
    let mut two_won: u64 = 0;

    let mut all_zeroes = false;
    while !all_zeroes {
        all_zeroes = true;
        let mut new_universes: HashMap<(usize, usize, usize, usize), u64> = HashMap::new();

        for (&(one_pos, one_score, two_pos, two_score), &num) in universes.iter() {
            all_zeroes = false;
            // add the new universes into the new universe array
            // the 3 dice rolls from player 1 can produce a value
            // between 3 to 9,
            // with these number of dice rolls causing each number:
            // 3  4  5  6  7  8  9
            // 1, 3, 6, 7, 6, 3, 1
            for (roll, n) in [0, 0, 0, 1, 3, 6, 7, 6, 3, 1].iter().enumerate().skip(3) {
                let new_pos = (roll + 9 + one_pos) % 10 + 1;
                let new_score = one_score + new_pos;
                if new_score >= 21 {
                    // add this to the number of universes where 1 won
                    one_won += num * n;
                } else {
                    // simulate player 2's roll
                    for (roll2, n2) in [0, 0, 0, 1, 3, 6, 7, 6, 3, 1].iter().enumerate().skip(3) {
                        let new_pos2 = (roll2 + 9 + two_pos) % 10 + 1;
                        let new_score2 = two_score + new_pos2;
                        if new_score2 >= 21 {
                            // add to the list of universes where player 2 won
                            two_won += num * n * n2;
                        } else {
                            // else add to the list of universes to simulate next round
                            let entry = new_universes.entry((new_pos, new_score, new_pos2, new_score2)).or_insert(0);
                            *entry += num * n * n2;
                        }
                    }
                }
            }
        }
        universes = new_universes;
        println!("{:?}", &universes);
    }
    println!("{} {}", one_won, two_won);
}
