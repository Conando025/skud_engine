use super::*;

#[derive(Clone)]
pub struct Grid {
    pub(super) cells: Vec<Cell>,
}

impl Grid {
    pub fn create(board: Board) -> Self {
        let mut grid = Grid {
            cells: vec![None; 289],
        };
        for (tile, pos) in &board.played_tiles_guest {
            *grid.index_mut(pos) = Some((*tile, Owner::Guest));
        }
        for (tile, pos) in &board.played_tiles_host {
            *grid.index_mut(pos) = Some((*tile, Owner::Host));
        }
        grid
    }

    pub(super) fn open_gates(&self) -> Vec<Position> {
        Position::GATES
            .into_iter()
            .filter(|pos| self.index(pos).is_none())
            .collect()
    }

    pub(super) fn index(&self, position: &Position) -> &Option<(Tile, Owner)> {
        let (x, y) = position.value();
        if x > 8 || y > 8 || x.abs() + y.abs() > 12 {
            panic!("Index out of bounds");
        } else {
            let (x, y) = (x + 8, y + 8);
            &self.cells[x as usize + y as usize * 17]
        }
    }
    pub(super) fn index_mut(&mut self, position: &Position) -> &mut Option<(Tile, Owner)> {
        let (x, y) = position.value();
        if x.abs() > 8 || y.abs() > 8 || x.abs() + y.abs() > 12 {
            panic!("Index out of bounds");
        } else {
            let (x, y) = (x + 8, y + 8);
            &mut self.cells[x as usize + y as usize * 17]
        }
    }
}
