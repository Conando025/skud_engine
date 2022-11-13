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
        for i in -8..8 {
            let mut last_column_tile_pos_option = None;
            let mut last_row_tile_pos_option = None;
            for j in -8..8 {
                let Some(pos) = Position::new(i, j) else {
                        continue ;
                    };
                if pos.is_gate() {
                    last_column_tile_pos_option = None;
                    continue;
                }
                let Some((new_column_tile_type, new_column_tile_owner)) = self.index(&pos) else {
                        continue ;
                    };
                let Some(last_column_tile_pos) = &last_column_tile_pos_option else {
                        last_column_tile_pos_option = Some(pos);
                        continue ;
                    };
                let (last_column_tile_type, last_column_tile_owner) =
                    self.index(last_column_tile_pos).clone().unwrap();
                let same_owner = *new_column_tile_owner == last_column_tile_owner;
                let tiles_harmonize = last_column_tile_type.harmonizes(new_column_tile_type);
                if same_owner && tiles_harmonize {
                    harmonie_list.push((
                        last_column_tile_owner,
                        last_column_tile_pos.clone(),
                        pos.clone(),
                    ));
                }
                last_column_tile_pos_option = Some(pos);
            }
            for j in -8..8 {
                let Some(pos) = Position::new(j, i) else {
                        continue ;
                    };
                if pos.is_gate() {
                    last_row_tile_pos_option = None;
                    continue;
                }
                let Some((new_row_tile_type, new_row_tile_owner)) = self.index(&pos) else {
                        continue ;
                    };
                let Some(last_row_tile_pos) = &last_row_tile_pos_option else {
                        last_row_tile_pos_option = Some(pos);
                        continue ;
                    };
                let (last_row_tile_type, last_row_tile_owner) =
                    self.index(last_row_tile_pos).clone().unwrap();
                let same_owner = *new_row_tile_owner == last_row_tile_owner;
                let tiles_harmonize = last_row_tile_type.harmonizes(new_row_tile_type);
                if same_owner && tiles_harmonize {
                    harmonie_list.push((
                        last_row_tile_owner,
                        last_row_tile_pos.clone(),
                        pos.clone(),
                    ));
                }
                last_row_tile_pos_option = Some(pos);
            }
        }
        harmonie_list
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("    ")?;
        for column in -8..=8 {
            write!(f, "  {:2}  ", column)?;
        }
        f.write_str("\n")?;
        for row in (0..17).rev() {
            write!(f, "{:<2}: ", row as isize - 8)?;
            for column in 0..17 {
                if let Some((Tile::Flower(flower), o)) = &self.cells[column + row * 17] {
                    write!(
                        f,
                        "[{} {}]",
                        match flower {
                            FlowerTile::Rose => "R3",
                            FlowerTile::Chrysanthemum => "R4",
                            FlowerTile::Rhododendron => "R5",
                            FlowerTile::Jasmine => "W3",
                            FlowerTile::Lily => "W4",
                            FlowerTile::WhiteJade => "W5",
                        },
                        match o {
                            &Owner::Host => "H",
                            &Owner::Guest => "G",
                        }
                    )?;
                } else {
                    f.write_str("[    ]")?;
                }
            }
            f.write_str("\n")?;
        }
        std::fmt::Result::Ok(())
    }
}
