pub struct Geometry{
    boardsize: u8,
    blocksize: u8,
    row_members: Vec<Vec<u8>>,
    col_members: Vec<Vec<u8>>,
    block_members: Vec<Vec<u8>>,
}

impl Geometry {
    pub fn new(blocksize: u8) -> Geometry {
        let boardsize = blocksize * blocksize;
        let mut result = Geometry{
            boardsize: boardsize,
            blocksize: blocksize,
            row_members: vec![vec![]; boardsize.into()],
            col_members: vec![vec![]; boardsize.into()],
            block_members: vec![vec![]; boardsize.into()],
        };
        for cell_id in 0 .. boardsize*boardsize {
            let row_id = result.get_row_id(cell_id);
            let col_id = result.get_col_id(cell_id);
            let block_id = result.get_block_id(cell_id);
            result.row_members[row_id as usize].push(cell_id);
            result.col_members[col_id as usize].push(cell_id);
            result.block_members[block_id as usize].push(cell_id);
        }
        return result
    }

    pub fn get_board_size(&self) -> u8 {
        self.boardsize
    }
    pub fn get_block_size(&self) -> u8 {
        self.blocksize
    }
    pub fn get_cell_count(&self) -> u16 {
        (self.boardsize as u16) * (self.boardsize as u16)
    }

    pub fn get_row_id(&self, cell_id: u8) -> u8 {
        cell_id / self.boardsize
    }
    pub fn get_col_id(&self, cell_id: u8) -> u8 {
        cell_id % self.boardsize
    }
    pub fn get_block_id(&self, cell_id: u8) -> u8 {
        let vertical_index = self.get_row_id(cell_id) / self.blocksize;
        let horizontal_index = self.get_col_id(cell_id) / self.blocksize;
        return vertical_index * self.blocksize + horizontal_index
    }

    pub fn get_row_members(&self, row_id: u8) -> &[u8] {
        &self.row_members[row_id as usize][..]
    }
    pub fn get_col_members(&self, col_id: u8) -> &[u8] {
        &self.col_members[col_id as usize][..]
    }
    pub fn get_block_members(&self, block_id: u8) -> &[u8] {
        &self.block_members[block_id as usize][..]
    }

    pub fn get_nonunique_neighbors(&self, cell_id: u8) -> impl Iterator<Item = &u8> {
        let row_iter = self.get_row_members(self.get_row_id(cell_id)).iter();
        let col_iter = self.get_col_members(self.get_col_id(cell_id)).iter();
        let block_iter = self.get_block_members(self.get_block_id(cell_id)).iter();
        let result = row_iter.chain(col_iter).chain(block_iter).filter(move |&&c| c!=cell_id);
        return result
    }
}
