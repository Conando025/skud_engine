use super::*;

#[derive(Clone)]
pub struct Board {
    pub(super) played_tiles_guest: Vec<(Tile, Position)>,
    pub(super) played_tiles_host: Vec<(Tile, Position)>,
    pub(super) reserve_host: Vec<(Tile, u8)>, //tiles and their counts
    pub(super) reserve_guest: Vec<(Tile, u8)>,
    pub(super) move_count: i16,
    pub(super) moves_since_planting: i16,
}

impl Board {
    pub fn empty() -> Self {
        Board {
            played_tiles_guest: Vec::with_capacity(10),
            played_tiles_host: Vec::with_capacity(10),
            reserve_host: vec![
                (Tile::Flower(FlowerTile::Rose), 3),
                (Tile::Flower(FlowerTile::Rhododendron), 3),
                (Tile::Flower(FlowerTile::Chrysanthemum), 3),
                (Tile::Flower(FlowerTile::Lily), 3),
                (Tile::Flower(FlowerTile::WhiteJade), 3),
                (Tile::Flower(FlowerTile::Jasmine), 3),
            ], //tiles and their counts
            reserve_guest: vec![
                (Tile::Flower(FlowerTile::Rose), 3),
                (Tile::Flower(FlowerTile::Rhododendron), 3),
                (Tile::Flower(FlowerTile::Chrysanthemum), 3),
                (Tile::Flower(FlowerTile::Lily), 3),
                (Tile::Flower(FlowerTile::WhiteJade), 3),
                (Tile::Flower(FlowerTile::Jasmine), 3),
            ],
            move_count: 0,
            moves_since_planting: 0,
        }
    }

    pub fn move_count(&self) -> i16 {
        self.move_count
    }

    pub fn moves_since_planting(&self) -> i16 {
        self.moves_since_planting
    }

    fn guest_add_tile(&mut self, tile: Tile, position: Position) {
        self.played_tiles_guest.push((tile, position));
        for (tile_type, amount) in &mut self.reserve_guest {
            if *tile_type == tile {
                *amount -= 1;
                break;
            }
        }
    }

    pub fn finished(&self, grid: &Grid, perspective: Player) -> Option<Output> {
        let harmonie_list = grid.list_all_harmonies();
        let mut guest_harmonies = Vec::new();
        let mut guest_crossing_harmonies = 0;
        let mut host_harmonies = Vec::new();
        let mut host_crossing_harmonies = 0;

        for harmonie in harmonie_list.into_iter() {
            let (owner, p1, p2) = harmonie;
            let ((p1_x, p1_y),(p2_x, p2_y)) = (p1.value(), p2.value());
            match owner {
                Owner::Host => {
                    if (p1_y as isize) * (p2_y as isize) < 0 || (p1_x as isize) * (p2_x as isize) < 0 {
                        host_crossing_harmonies += 1;
                    }
                    host_harmonies.push((p1,p2));
                }
                Owner::Guest => {
                    if (p1_y as isize) * (p2_y as isize) < 0 || (p1_x as isize) * (p2_x as isize) < 0 {
                        guest_crossing_harmonies += 1;
                    }
                    guest_harmonies.push((p1,p2));
                }
            }
        }

        todo!("Harmony rings");

        if self.moves_since_planting >= 50 {
            return Some(Output::Draw);
        };
        return None;

    }

    pub fn all_legal_moves(&self, grid: &mut Grid) -> Moves {
        let mut move_set: Moves = Vec::new();
        if self.finished(grid, next_to_move()).is_some() {
            return move_set;
        }

        let (tiles_played, reserve) = match self.next_to_move() {
            Player::Guest => {
                (&self.played_tiles_guest, &self.reserve_guest)
            }
            Player::Host => {
                (&self.played_tiles_host, &self.reserve_host)
            }
        };
        for (tile, position) in tiles_played {
            let mut moves_for_piece =
                all_possibilities_for_piece_to_move(grid, *tile, position.clone());
            move_set.append(&mut moves_for_piece);
        }

        for gate in grid.open_gates() {
            for (Tile::Flower(flower), amount) in reserve {
                if *amount > 0 {
                    move_set.push(Move::Planting(*flower, gate.clone()));
                }
            }
        }
        move_set
    }

    fn host_add_tile(&mut self, tile: Tile, position: Position) {
        self.played_tiles_host.push((tile, position));
        for (tile_type, amount) in &mut self.reserve_host {
            if *tile_type == tile {
                *amount -= 1;
                break;
            }
        }
    }

    pub fn next_to_move(&self) -> Player {
        if self.move_count % 2 == 0 {
            Player::Guest
        } else {
            Player::Host
        }
    }

    pub fn apply_move(&mut self, a_move: Move) {
        match a_move {
            Move::Planting(flower_tile, position) => {
                self.moves_since_planting = 0;
                match self.next_to_move() {
                    Player::Host => {
                        self.host_add_tile(Tile::Flower(flower_tile), position);
                    }
                    Player::Guest => {
                        self.guest_add_tile(Tile::Flower(flower_tile), position);
                    }
                }
            }
            Move::Arranging(start, end) => {
                self.moves_since_planting += 1;
                if self.move_count % 2 == 0 {
                    //Turn Guest
                    for (_tile_type, position) in &mut self.played_tiles_guest {
                        if *position == start {
                            *position = end;
                            break;
                        }
                    }
                } else {
                    //Turn Host
                    for (_tile_type, position) in &mut self.played_tiles_guest {
                        if *position == start {
                            *position = end;
                            break;
                        }
                    }
                }
            }
        }
        self.move_count += 1;
    }
}
