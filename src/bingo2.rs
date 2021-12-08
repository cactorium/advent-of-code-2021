type Board = [[usize; 5]; 5];

fn won(board: &Board, nums: &[usize]) -> bool {
    let row_full = (0..5).any(|row| {
        (0..5).all(|col| nums.iter().any(|n| *n == board[row][col]))
    });
    let col_full = (0..5).any(|col| {
        (0..5).all(|row| nums.iter().any(|n| *n == board[row][col]))
    });
    row_full || col_full
}

fn main() {
    let stdin = std::io::stdin();

    let mut numbers_line = String::new();
    stdin.read_line(&mut numbers_line).unwrap();
    let numbers: Vec<_> = numbers_line.trim()
        .split(",")
        .map(|s| str::parse::<usize>(s.trim()).unwrap())
        .collect();

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap(); // extra newline
    buffer.clear();

    let mut boards = Vec::new();
    let mut cur_board: Board = [[0usize; 5]; 5];
    let mut done = false;
    while !done {
        for i in 0..5 {
            let read_result = stdin.read_line(&mut buffer).unwrap();
            if read_result == 0 {
                done = true;
                break;
            }

            let numbers: Vec<_> = buffer
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| str::parse::<usize>(s.trim()).unwrap())
                .collect();
            if numbers.len() != 5 {
                println!("invalid line {:?}", buffer);
                break;
            }
            for j in 0..5 {
                cur_board[i][j] = numbers[j];
            }

            buffer.clear();
        }

        boards.push(cur_board);
        let result = stdin.read_line(&mut buffer).unwrap();
        if result == 0 {
            break;
        }

        buffer.clear();
    }
    println!("boards {:?}", &boards);
    println!("numbers {:?}", &numbers);

    for i in 1..numbers.len() {
        let cur_nums = &numbers[0..i];
        let last_nums = &numbers[0..(i-1)];

        let won_boards: Vec<_> = boards.iter().filter(|b| won(b, cur_nums)).collect();
        let unwon_boards: Vec<_> = won_boards.iter().filter(|b| !won(b, last_nums)).collect();

        if won_boards.len() == boards.len() && unwon_boards.len() == 1 {
            println!("last board: {:?}", unwon_boards[0]);
            let unmarked_sum: usize = unwon_boards[0].iter()
                .map(|row| {
                    row.iter()
                        .filter(|n| cur_nums.iter().all(|m| *m != **n))
                        .sum::<usize>()
                })
                .sum();
            println!("score {}", unmarked_sum * cur_nums[cur_nums.len() - 1]);
            return;
        }
    }
}
