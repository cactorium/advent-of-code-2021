fn main() {
    let mut one_score = 0;
    let mut two_score = 0;

    let mut one_position = 6;
    let mut two_position = 1;

    let mut num_rolls = 0;

    while one_score < 1000 && two_score < 1000 {
        let one_moves = (num_rolls * 3) + 6;
        one_position = (one_position + one_moves + 9) % 10 + 1;
        one_score += one_position;
        num_rolls += 3;

        if one_score >= 1000 {
            break;
        }

        let two_moves = (num_rolls * 3) + 6;
        two_position = (two_position + two_moves + 9) % 10 + 1;
        two_score += two_position;
        num_rolls += 3;
        //println!("{} {}", one_score, two_score);
    }

    let losing = if one_score < 1000 { one_score } else { two_score };
    println!("{}", losing*num_rolls);
}
