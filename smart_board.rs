use super::geometry::*;


#[derive(Clone)]
pub struct SmartBoard<'a> {
    geometry: &'a Geometry,
    // TODO describe each value
    possibility: Vec<Vec<bool>>,
    cell_values_count: Vec<u8>,
    row_position_count: Vec<Vec<u8>>,
    col_position_count: Vec<Vec<u8>>,
    block_position_count: Vec<Vec<u8>>,
}


impl SmartBoard <'_>{
    pub fn new(geometry: &Geometry) -> SmartBoard{
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

    pub fn set(&mut self, cell_id: u8, value: u8) -> bool {
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

    pub fn unset(&mut self, cell_id: u8, value: u8) -> bool {
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

    pub fn extract_cell_values(&self) -> Vec<Vec<Option<usize>>> {
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

    pub fn find_a_guess(&self) -> Option<(u8, u8)> {
        for cell_id in 0..self.geometry.get_cell_count() {
            if self.cell_values_count[cell_id as usize] > 1 {
                for value in 0..self.geometry.get_board_size() {
                    if self.possibility[cell_id as usize][value as usize] {
                        return Some((cell_id as u8, value));
                    }
                }
            }
        }
        return None;
    }
}
