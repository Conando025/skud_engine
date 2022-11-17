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

    pub fn apply_move(
        &mut self,
        a_move: Move,
        player: Player,
        harmonie_list: &mut Vec<(Owner, Position, Position)>,
    ) {
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
                let cell_content = self.index(&start).clone();
                *self.index_mut(&end) = cell_content.clone();
                *self.index_mut(&start) = None;
                for (index, (_, from, to)) in harmonie_list.clone().into_iter().enumerate().rev() {
                    if from == start || to == start {
                        #[cfg(debug_assertions)]
                        println!("removing harmony due to tile being moved. Harmony was from {from:?} to {to:?}");
                        harmonie_list.remove(index);
                        continue;
                    }
                    let from = from.value();
                    let to = to.value();
                    let end = end.value();
                    if from.0 == to.0 {
                        if (from.1 > end.1 && end.1 > to.1) || (from.1 < end.1 && end.1 < to.1) {
                            #[cfg(debug_assertions)]
                            println!("removing harmony due to new tile position blocking it. Harmony was from {from:?} to {to:?}");
                            harmonie_list.remove(index);
                            continue;
                        }
                    } else {
                        if (from.0 > end.0 && end.0 > to.0) || (from.0 < end.0 && end.0 < to.0) {
                            #[cfg(debug_assertions)]
                            println!("removing harmony due to new tile position blocking it. Harmony was from {from:?} to {to:?}");
                            harmonie_list.remove(index);
                            continue;
                        }
                    }
                }
                let Some(cell_content) = cell_content else {
                    panic!("What the?");
                };
                //check for harmonies at the new position
                let mut tiles_in_direction: [Option<(Tile, Owner)>; 4] = [None; 4];
                let mut pos_in_direction: [Option<Position>; 4] = [None, None, None, None];
                for (i, d) in Direction::ALL.into_iter().enumerate() {
                    pos_in_direction[i] = end.add(d);
                    while let Some(new_pos) = &pos_in_direction[i] {
                        if new_pos.is_gate() {
                            break;
                        }
                        if let Some(c) = self.index(&new_pos) {
                            tiles_in_direction[i] = Some(*c);
                            break;
                        }
                        pos_in_direction[i] = new_pos.add(d);
                    }
                }
                for (index, c) in tiles_in_direction.into_iter().enumerate() {
                    let Some((t, o)) = c else {
                        continue
                    };
                    if t.harmonizes(&cell_content.0) && cell_content.1 == o {
                        let from = end.clone();
                        let to = pos_in_direction[index].clone().unwrap();
                        #[cfg(debug_assertions)]
                        println!("adding harmony involving new tile position. From {from:?} to {to:?}");
                        harmonie_list.push((
                            o,
                            from,
                            to,
                        ))
                    }
                }
                //check for harmonies in the position it used to block
                if !start.is_gate() {
                    let mut tiles_in_direction: [Option<(Tile, Owner)>; 4] = [None; 4];
                    let mut pos_in_direction: [Option<Position>; 4] = [None, None, None, None];
                    for (i, d) in Direction::ALL.into_iter().enumerate() {
                        pos_in_direction[i] = start.add(d);
                        while let Some(new_pos) = &pos_in_direction[i] {
                            if new_pos.is_gate() || *new_pos == end {
                                break;
                            }
                            if let Some(c) = self.index(&new_pos) {
                                tiles_in_direction[i] = Some(*c);
                                break;
                            }
                            pos_in_direction[i] = new_pos.add(d);
                        }
                    }
                    if let (Some(up), Some(down)) = (tiles_in_direction[0], tiles_in_direction[1]) {
                        if up.1 == down.1 && up.0.harmonizes(&down.0) {
                            let from = pos_in_direction[0].clone().unwrap();
                            let to = pos_in_direction[1].clone().unwrap();
                            #[cfg(debug_assertions)]
                            println!("adding harmony that used to be blocked. From {from:?} to {to:?}");
                            harmonie_list.push((
                                up.1,
                                from,
                                to,
                            ));
                        }
                    }
                    if let (Some(left), Some(right)) = (tiles_in_direction[2], tiles_in_direction[3]) {
                        if left.1 == right.1 && left.0.harmonizes(&right.0) {
                            let from = pos_in_direction[2].clone().unwrap();
                            let to = pos_in_direction[3].clone().unwrap();
                            #[cfg(debug_assertions)]
                            println!("adding harmony that used to be blocked. From {from:?} to {to:?}");
                            harmonie_list.push((
                                left.1,
                                from,
                                to,
                            ));
                        }
                    }
                }
            }
        }
    }

    pub fn list_all_harmonies(&self) -> Vec<(Owner, Position, Position)> {
        let mut harmonie_list = Vec::new();
        for i in -8..8 {
            self.add_all_harmonies_row(i, &mut harmonie_list);
            self.add_all_harmonies_column(i, &mut harmonie_list);
        }
        harmonie_list
    }

    fn add_all_harmonies_row(&self, row: i8, harmonie_list: &mut Vec<(Owner, Position, Position)>) {

        let mut last_column_tile_pos_option = None;

        for j in -8..8 {
            let Some(pos) = Position::new(row, j) else {
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
    }


    fn add_all_harmonies_column(&self, column: i8, harmonie_list: &mut Vec<(Owner, Position, Position)>){
        let mut last_row_tile_pos_option = None;
        for j in -8..8 {
            let Some(pos) = Position::new(j, column) else {
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
