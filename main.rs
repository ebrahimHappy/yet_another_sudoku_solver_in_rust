use std::fmt;

mod smart_board;
mod geometry;
mod backtrack;


fn main(){
    let raw_problem = [
        [5,3,1,0,4,0,0,0,0],
        [0,0,0,0,0,5,0,8,0],
        [0,0,7,0,0,0,0,0,4],
        [9,6,0,0,0,0,5,0,1],
        [1,0,5,0,9,0,0,0,6],
        [0,0,0,0,1,6,0,0,0],
        [0,9,6,0,2,0,0,0,0],
        [0,0,0,7,5,4,0,3,9],
        [0,0,0,0,0,9,4,0,8],
    ];
    let problem = raw_problem.iter().map(|r| r.iter().map(|&v| if v == 0 {None} else {Some(v as u8)}).collect()).collect();
    let solutions = backtrack::solve(problem, 2);
    for solution in solutions {
        for row in solution {
            for value in row{
                print!("{} ", value);
            }
            println!{""};
        }
        println!("");
    }
}
