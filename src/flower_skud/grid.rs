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
        let (x, y) = (x + 8, y + 8);
        &self.cells[x as usize + y as usize * 17]
    }

    pub(super) fn index_mut(&mut self, position: &Position) -> &mut Option<(Tile, Owner)> {
        let (x, y) = position.value();
        let (x, y) = (x + 8, y + 8);
        &mut self.cells[x as usize + y as usize * 17]
    }

    pub fn apply_move(&mut self, a_move: Move, player: Player) {
        match a_move {
            Move::Planting(flower_tile, position) => {
                *self.index_mut(&position) = Some((
                    Tile::Flower(flower_tile),
                    match player {
                        Player::Host => Owner::Host,
                        Player::Guest => Owner::Guest,
                    },
                ));
            }
            Move::Arranging(start, end) => {
                let begin_content = self.index(&start).clone();
                *self.index_mut(&end) = begin_content;
                *self.index_mut(&start) = None
            }
        }
    }

    pub(super) fn list_all_harmonies(&self) -> Vec<(Owner, Position, Position)> {
        let mut harmonie_list = Vec::new();
        for i in 0..17 {
            let mut last_column_tile: &Cell = &None;
            let mut last_column_tile_coords = (0,0);
            let mut last_row_tile = None;
            let mut last_row_tile_coords = (0,0);
            for j in 0..17 {
                let (row,column) = (i,j);
                let new_column_tile = &self.cells[row + column * 17];
                if self::foo(last_column_tile, new_column_tile) {
                    harmonie_list.push((last_column_tile.unwrap().1, Position::new(last_column_tile_coords.0,last_column_tile_coords.1).unwrap(), Position::new(i as i8 - 8,j as i8 -8).unwrap()));
                }
                let (column, row) = (i,j);
                let new_row_tile = &self.cells[row + column * 17];
                if self::foo(last_row_tile, new_row_tile) {
                    harmonie_list.push((last_row_tile.unwrap().1, Position::new(last_row_tile_coords.0,last_row_tile_coords.1).unwrap(), Position::new(i as i8 - 8,j as i8 -8).unwrap()));
                }
            }
        }
        harmonie_list
    }

    fn foo(last_column_tile: &mut Cell, new_column_tile: &Cell) -> bool {
        if let Some((new_column_tile_type, new_column_tile_owner)) = new_column_tile {
            if let Some((last_column_tile_type, last_column_tile_owner)) = last_column_tile {
                if *new_column_tile_owner == *last_column_tile_owner {
                    last_column_tile_type.harmonizes(new_column_tile_type)
                }
            }
            *last_column_tile = new_column_tile.clone();
        }
        false
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut representation: String = "".to_owned();
        for row in 0..17 {
            representation += "[ ";
            for column in 0..17 {
                representation += " [ ";
                if let Some((Tile::Flower(flower), o)) = &self.cells[row + column * 17] {
                    match flower {
                        FlowerTile::Rose => {
                            representation += " R3";
                        }
                        FlowerTile::Chrysanthemum => {
                            representation += " R4";
                        }
                        FlowerTile::Rhododendron => {
                            representation += " R5";
                        }
                        FlowerTile::Jasmine => {
                            representation += " W3";
                        }
                        FlowerTile::Lily => {
                            representation += " W4";
                        }
                        FlowerTile::WhiteJade => {
                            representation += " W5";
                        }
                    }
                    match o {
                        Owner::Host => {
                            representation += ",H ";
                        }
                        Owner::Guest => {
                            representation += ",G ";
                        }
                    }
                } else {
                    representation += "      "
                }
                representation += "]";
            }
            representation += " ]\n";
        }
        write!(f, "{}", representation)
    }
}
