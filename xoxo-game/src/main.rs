use std::io;

fn print_board(board: &Vec<char>) {
    println!("{} | {} | {}", board[0], board[1], board[2]);
    println!("-----------");
    println!("{} | {} | {}", board[3], board[4], board[5]);
    println!("-----------");
    println!("{} | {} | {}", board[6], board[7], board[8]);
}

fn main() {
    let mut board = vec![' '; 9];
    let mut current_player = 'X';
    let mut move_count = 0;

    loop {
        print_board(&board);
        println!("Player {}: Enter your move (1-9):", current_player);

        let mut move_input = String::new();
        io::stdin().read_line(&mut move_input).unwrap();
        let move_input: u32 = match move_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number from 1 to 9.");
                continue;
            }
        };

        if move_input < 1 || move_input > 9 || board[move_input as usize - 1] != ' ' {
            println!(
                "Invalid move. Please enter a valid number from 1 to 9 that is not already taken."
            );
            continue;
        }

        board[move_input as usize - 1] = current_player;
        move_count += 1;

        if move_count >= 5 {
            let lines = [
                [0, 1, 2],
                [3, 4, 5],
                [6, 7, 8],
                [0, 3, 6],
                [1, 4, 7],
                [2, 5, 8],
                [0, 4, 8],
                [2, 4, 6],
            ];

            for line in lines.iter() {
                if board[line[0]] == current_player
                    && board[line[1]] == current_player
                    && board[line[2]] == current_player
                {
                    print_board(&board);
                    println!("Player {} wins!", current_player);
                    return;
                }
            }
        }

        if move_count == 9 {
            println!("It's a tie!");
            return;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}
