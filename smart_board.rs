use std::fmt;

mod geometry;
use crate::geometry::*;


struct SmartBoard<'a> {
    geometry: &'a Geometry,
    // TODO describe each value
    possibility: Vec<Vec<bool>>,
    cell_values_count: Vec<u8>,
    row_position_count: Vec<Vec<u8>>,
    col_position_count: Vec<Vec<u8>>,
    block_position_count: Vec<Vec<u8>>,
}


impl SmartBoard <'_>{
    fn new(geometry: &Geometry) -> SmartBoard{
        // vec![vec![]; boardsize.into()]
        let cell_count = geometry.get_cell_count();
        let board_size = geometry.get_board_size();
        let value_count = geometry.get_board_size();
        return SmartBoard{
            geometry: geometry,
            possibility: vec![vec![true; value_count.into()]; cell_count.into()],
            cell_values_count: vec![value_count; cell_count.into()],
            row_position_count: vec![vec![board_size; value_count.into()]; board_size.into()],
            col_position_count: vec![vec![board_size; value_count.into()]; board_size.into()],
            block_position_count: vec![vec![board_size; value_count.into()]; board_size.into()],
        }
    }

    fn set(&mut self, cell_id: u8, value: u8) -> bool {
        // TODO return immediately if it is set before
        if !self.possibility[cell_id as usize][value as usize] {
            return false;  // Impossible
        }

        for v in 0..self.geometry.get_board_size() {
            if v != value {
                if !self.unset(cell_id, v) {
                    return false;
                }
            }
        }
        for &c in self.geometry.get_nonunique_neighbors(cell_id) {
            if !self.unset(c, value) {
                return false;
            }
        }
        return true;
    }

    fn unset(&mut self, cell_id: u8, value: u8) -> bool {
        if !self.possibility[cell_id as usize][value as usize] {
            return true;  // No action needed
        }

        self.possibility[cell_id as usize][value as usize] = false;

        let row_id = self.geometry.get_row_id(cell_id);
        let col_id = self.geometry.get_col_id(cell_id);
        let block_id = self.geometry.get_block_id(cell_id);

        self.cell_values_count[cell_id as usize] -= 1;
        self.row_position_count[row_id as usize][value as usize] -= 1;
        self.col_position_count[col_id as usize][value as usize] -= 1;
        self.block_position_count[block_id as usize][value as usize] -= 1;

        if self.cell_values_count[cell_id as usize] == 0 ||
           self.row_position_count[row_id as usize][value as usize] == 0 ||
           self.col_position_count[col_id as usize][value as usize] == 0 ||
           self.block_position_count[block_id as usize][value as usize] == 0
        {
            return false;
        }

        if self.cell_values_count[cell_id as usize] == 1 {
            for v in 0..self.geometry.get_board_size() {
                if self.possibility[cell_id as usize][v as usize] {
                    if !self.set(cell_id, v) {
                        return false;
                    }
                }
            }
        }
        if self.row_position_count[row_id as usize][value as usize] == 1 {
            for &c in self.geometry.get_row_members(row_id) {
                if self.possibility[c as usize][value as usize] {
                    if !self.set(c, value) {
                        return false;
                    }
                }
            }
        }
        if self.col_position_count[col_id as usize][value as usize] == 1 {
            for &c in self.geometry.get_col_members(col_id) {
                if self.possibility[c as usize][value as usize] {
                    if !self.set(c, value) {
                        return false;
                    }
                }
            }
        }
        if self.block_position_count[block_id as usize][value as usize] == 1 {
            for &c in self.geometry.get_block_members(block_id) {
                if self.possibility[c as usize][value as usize] {
                    if !self.set(c, value) {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    fn extract_cell_values(&self) -> Vec<Vec<Option<usize>>> {
        let board_size = self.geometry.get_board_size() as usize;
        let mut result: Vec<Vec<Option<usize>>> = vec![vec![None; board_size]; board_size];
        for i in 0..board_size {
            for j in 0..board_size {
                let cell_id = (i*board_size+j) as usize;
                if self.cell_values_count[cell_id] == 1 {
                    for v in 0..board_size {
                        if self.possibility[cell_id][v as usize] {
                            result[i][j] = Some(v);
                        }
                    }
                }
            }
        }
        return result;
    }
}


fn main(){
    let geom = Geometry::new(3);
    let mut board = SmartBoard::new(&geom);
    
    // let problem = [
    //     [5,3,1,0,4,0,0,0,0],
    //     [0,0,0,0,0,5,0,8,0],
    //     [0,0,7,0,0,0,0,0,4],
    //     [9,6,0,0,0,0,5,0,1],
    //     [1,0,5,0,9,0,0,0,6],
    //     [0,0,0,0,1,6,0,0,0],
    //     [0,9,6,0,2,0,0,0,0],
    //     [0,0,0,7,5,4,0,3,9],
    //     [0,0,0,0,0,9,4,0,8 as u8],
    // ];
    let problem = [
        [0,6,9,0,5,0,3,0,0],
        [0,8,1,0,9,3,0,0,5],
        [0,0,5,4,8,0,0,1,0],
        [9,2,6,0,0,0,7,0,8],
        [0,5,0,0,0,0,0,4,9],
        [0,0,0,0,0,9,6,0,1],
        [0,0,4,0,3,8,0,2,7],
        [0,0,0,0,4,5,0,0,0],
        [5,1,0,2,7,6,8,0,4 as u8],
    ];

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
    // println!("{}", geom.get_row_id(9));
    // println!("{}", geom.get_col_id(9));
    // println!("{}", geom.get_block_id(40));
    // for c in geom.get_nonunique_neighbors(0){
    //     println!("{}", c)
    // }

}