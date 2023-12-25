use std::fmt;

mod geometry;
mod smart_board;
use crate::geometry::*;
use crate::smart_board::*;


fn solve(board: &mut SmartBoard) -> bool {
    match board.find_a_guess() {
        None => print_board(board),
        Some((cell_id, value)) => {
            let mut new_board = board.clone();
            println!("clone!");
            if new_board.set(cell_id, value){
                if solve(&mut new_board) {
                    return true;
                }
            }
            if board.unset(cell_id, value) == false {
                return false;
            }
            return solve(board);
        },
    }
    return true
}


fn print_board(board: &SmartBoard) {
    let result = board.extract_cell_values();
    for i in 0..9 {
        for j in 0..9 {
            match result[i][j] {
                None => print!(" "),
                Some(v) => print!("{}", v+1),
            }
        }
        println!("")
    }
}


fn main(){
    let geom = Geometry::new(3);
    let mut board = SmartBoard::new(&geom);
    
    let problem = [
        [5,3,1,0,4,0,0,0,0],
        [0,0,0,0,0,5,0,8,0],
        [0,0,7,0,0,0,0,0,4],
        [9,6,0,0,0,0,5,0,1],
        [1,0,5,0,9,0,0,0,6],
        [0,0,0,0,1,6,0,0,0],
        [0,9,6,0,2,0,0,0,0],
        [0,0,0,7,5,4,0,3,9],
        [0,0,0,0,0,9,4,0,8 as u8],
    ];
    // let problem = [
    //     [0,6,9,0,5,0,3,0,0],
    //     [0,8,1,0,9,3,0,0,5],
    //     [0,0,5,4,8,0,0,1,0],
    //     [9,2,6,0,0,0,7,0,8],
    //     [0,5,0,0,0,0,0,4,9],
    //     [0,0,0,0,0,9,6,0,1],
    //     [0,0,4,0,3,8,0,2,7],
    //     [0,0,0,0,4,5,0,0,0],
    //     [5,1,0,2,7,6,8,0,4 as u8],
    // ];

    for i in 0..9 {
        for j in 0..9 {
            if problem[i][j] > 0 {
                let cell_id = (i*9+j) as u8;
                if !board.set(cell_id, problem[i][j]-1) {
                    println!("oh no :(");
                }
            }
        }
    }

    print_board(&board);
    println!("=============");
    solve(&mut board);
    // println!("{}", geom.get_row_id(9));
    // println!("{}", geom.get_col_id(9));
    // println!("{}", geom.get_block_id(40));
    // for c in geom.get_nonunique_neighbors(0){
    //     println!("{}", c)
    // }
    // println!("{:?}", board.find_a_guess())
}