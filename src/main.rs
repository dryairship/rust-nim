use std::io::{self, Write};
use rand::Rng;

const MAX_SIZE: usize = 10;

#[derive(Debug)]
enum Player {
    Computer,
    Human,
}

impl Player {
    fn switch(&mut self) {
        match self {
            Player::Computer => *self = Player::Human,
            Player::Human => *self = Player::Computer,
        }
    }
}

fn get_usize_from_user_input(prompt: &str) -> usize {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: usize = input.trim().parse().expect("Not a number.");
    input
}

fn get_number_of_columns() -> usize {
    get_usize_from_user_input("Enter number of columns : ")
}

fn clear_screen() {
    println!("\x1B[2J");
}

fn print_game(game: &Vec<usize>) {
    let n = game.len();
    
    println!("\n");
    for i in 0..n {
        print!("\t{}", i+1);
    }
    println!("\n");
    for i in 0..MAX_SIZE {
        for j in 0..n {
            if game[j] >= MAX_SIZE-i {
                print!("\t#");
            } else {
                print!("\t ");
            }
        }
        println!("");
    }
    for _i in 0..n {
        print!("\t-");
    }
    println!("");
    for i in 0..n {
        print!("\t{}", game[i]);
    }
    println!("");
}

fn initialize_game(n: usize) -> Vec<usize> {
    let mut game = vec![0; n];
    let mut rng = rand::thread_rng();
    
    for i in 0..n {
        game[i] = rng.gen_range(1, MAX_SIZE+1);
    }
    game
}

fn display_finish_message(loser: Player) {
    match loser {
        Player::Human => println!("You lost! Better luck next time."),
        Player::Computer => println!("Congratulations! You won the game!"),
    }
}

fn check_game_over(game: &Vec<usize>) -> bool {
    for column in game.iter() {
        if *column != 0 {
            return false;
        }
    }
    return true;
}

fn get_computer_move(game: &Vec<usize>) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    
    let mut column: usize;
    loop {
        column = rng.gen_range(0, game.len());
        if game[column] > 0 {
            break;
        }
    }
    
    let nim: usize = rng.gen_range(1, game[column]+1);
    
    println!("Computer's move.");
    println!("Computer removed {} #s from column {}.", nim, column+1);
    (column, nim)
}

fn get_human_move(game: &Vec<usize>) -> (usize, usize) {
    println!("Your move.");
    let mut column: usize;
    loop {
        column = get_usize_from_user_input("Enter the column number to remove #s from : ");
        if column > game.len() {
            println!("{} is not a valid column number.", column);
            continue;
        } else if game[column-1] == 0 {
            println!("The column is already empty.");
            continue;
        }
        break;
    }
    column = column - 1;
    
    let mut nim: usize;
    loop {
        nim = get_usize_from_user_input("Enter the number of #s to remove : ");
        if game[column] < nim {
            println!("The column has only {} #. Enter a number less than or equal to that.", game[column]);
            continue;
        } else if nim <= 0 {
            println!("Enter a number greater than 0.");
            continue;
        }
        break;
    }
    (column, nim)
}

fn main() {
    let n = get_number_of_columns();
    let mut game = initialize_game(n);
    
    let mut player: Player = if rand::random() {
        Player::Human
    } else {
        Player::Computer
    };
    
    loop {
        print_game(&game);
        if check_game_over(&game) {
            display_finish_message(player);
            break;
        }
        let next_move = match player {
            Player::Computer => get_computer_move(&game),
            Player::Human => get_human_move(&game),
        };
        game[next_move.0] -= next_move.1;
        player.switch();
    }
}
