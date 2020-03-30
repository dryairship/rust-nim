use std::io::{self, Write};
use rand::Rng;

const MAX_SIZE: usize = 10;

fn get_number_of_columns() -> usize {
    print!("Enter number of columns : ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().expect("Not a number.");
    n
}

fn initialize_game(n: usize) -> Vec<usize> {
    let mut game = vec![0; n];
    let mut rng = rand::thread_rng();
    
    for i in 0..n {
        game[i] = rng.gen_range(1, MAX_SIZE+1);
    }
    game
}

fn main() {
    let n = get_number_of_columns();
    let game = initialize_game(n);
    println!("{:?}", game);
}


