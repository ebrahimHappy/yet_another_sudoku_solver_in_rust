
use crate::geometry::*;
use crate::smart_board::*;


fn backtrack(board: &mut SmartBoard, max_number_of_answers: usize) -> Vec<Vec<Vec<Option<usize>>>> {
    if max_number_of_answers <= 0 {
        return vec![];
    }
    match board.find_a_guess() {
        None => return vec![board.extract_cell_values()],
        Some((cell_id, value)) => {
            let mut answers = vec![];
            let mut new_board = board.clone();
            if new_board.set(cell_id, value){
                answers.extend(backtrack(&mut new_board, max_number_of_answers - answers.len()));
            }
            if board.unset(cell_id, value){
                answers.extend(backtrack(board, max_number_of_answers - answers.len()));
            }
            return answers;
        },
    }
}


pub fn solve(problem: Vec<Vec<Option<u8>>>, max_number_of_answers: usize) -> Vec<Vec<Vec<u8>>> {
    let geometry = Geometry::new(problem.len() as u8).unwrap();
    let mut board = SmartBoard::new(&geometry);
    for (i, row) in problem.iter().enumerate(){
        // TODO check len
        for (j, value) in row.iter().enumerate(){
            if let Some(number) = value {
                if board.set(geometry.get_cell_id_at(i as u8, j as u8), number - 1) == false{
                    return vec![];
                }
            }
        }
    }
    let solutions = backtrack(&mut board, max_number_of_answers);
    return solutions.iter().map(
        |s| s.iter().map(
            |r| r.iter().map(
                |c| c.unwrap() as u8 + 1
            ).collect()
        ).collect()
    ).collect();
}

