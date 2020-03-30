# rust-nim
This is a game of Nim against the computer.
You start with `n` (input from the user) number of columns that each have a random number of `#`s (Max 10).
The first player is randomly decided.

On your move, you can choose any column and remove a positive number of `#`s from it.

The objective of the game is to leave the opponent with no valid moves.

## Building
```
cargo build
```

## Running
Run the executable generated by the build command, or do :
```
cargo run
```