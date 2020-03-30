use std::io::{self, Write};
use rand::Rng;

const MAX_SIZE: usize = 10;

#[derive(Debug)]
enum Player {
    Computer,
    Human
}

fn get_number_of_columns() -> usize {
    print!("Enter number of columns : ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().expect("Not a number.");
    n
}

fn clear_screen() {
    println!("\x1B[2J");
}

fn print_game(game: &Vec<usize>) {
    clear_screen();
    let n = game.len();
    
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
        Player::Computer => println!("Congratulations! You won the game!")
    }
}

fn main() {
    let n = get_number_of_columns();
    let game = initialize_game(n);
    
    let player: Player;
    if rand::random() {
        player = Player::Human
    } else {
        player = Player::Computer;
    }
    
    display_finish_message(player);
}
