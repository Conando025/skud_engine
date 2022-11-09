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
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut representation: String = "".to_owned();
        for row in 0..17 {
            representation += "[ ";
            for column in 0..17 {
                representation += " [ ";
                if let Some((Tile::Flower(flower), o)) = &self.cells[row * 17 + column] {
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
