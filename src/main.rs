use std::io::{self, Write};

fn get_number_of_columns() -> usize {
    print!("Enter number of columns : ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().expect("Not a number.");
    n
}


fn main() {
    let n = get_number_of_columns();
    println!("{}", n);
}


