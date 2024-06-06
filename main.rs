use std::fmt;
use std::fs::read_to_string;

mod smart_board;
mod geometry;
mod backtrack;


fn parse_9x9(input: &String) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for character in input.chars(){
        if character == '.'{
            result.push(0)
        } else if let Some(num) = character.to_digit(10) { 
            result.push(num as u8)
        }
    }
    return result
}


fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("you need to pass a file name")
    }
    let problem = parse_9x9(&read_to_string(&args[1]).unwrap());

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
