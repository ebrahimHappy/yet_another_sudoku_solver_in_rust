# Yet Another Sudoku Solver in Rust
This is just a hobby project to learn rust. Don't expect it to be the fastest nor cleanest one, but it gets the job done. It solves a hard sudoku in a few milliseconds and it is just a few hundreds of lines.

## Build
```
git clone https://github.com/ebrahimHappy/yet_another_sudoku_solver_in_rust.git
cd yet_another_sudoku_solver_in_rust
rustc main.rs
```

## Run
```
./main problems/hard.txt
```

## Todo
- Break smart board into multiple classes (board and strategy).
- Use a idiomatic way to handle unsolvable cases.
- Improve source layout.
- Parse 16x16 problems.
- Advanced argument parsing.
