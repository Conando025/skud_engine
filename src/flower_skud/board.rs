use super::*;
use rand::{thread_rng, Rng};
use std::ops::IndexMut;

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
                (Tile::Flower(FlowerTile::Chrysanthemum), 3),
                (Tile::Flower(FlowerTile::Rhododendron), 3),
                (Tile::Flower(FlowerTile::Jasmine), 3),
                (Tile::Flower(FlowerTile::Lily), 3),
                (Tile::Flower(FlowerTile::WhiteJade), 3),
            ], //tiles and their counts
            reserve_guest: vec![
                (Tile::Flower(FlowerTile::Rose), 3),
                (Tile::Flower(FlowerTile::Chrysanthemum), 3),
                (Tile::Flower(FlowerTile::Rhododendron), 3),
                (Tile::Flower(FlowerTile::Jasmine), 3),
                (Tile::Flower(FlowerTile::Lily), 3),
                (Tile::Flower(FlowerTile::WhiteJade), 3),
            ],
            move_count: 0,
            moves_since_planting: 0,
        }
    }

    pub fn create_test() -> Self {
        let mut b = Self::empty();
        b.apply_move(Move::Planting(FlowerTile::Chrysanthemum, Position::new(0, -8).unwrap()));
        b.apply_move(Move::Planting(FlowerTile::Chrysanthemum, Position::new(0, 8).unwrap()));
        b.apply_move(Move::Planting(FlowerTile::Rhododendron, Position::new(8, 0).unwrap()));
        b.apply_move(Move::Planting(FlowerTile::Rhododendron, Position::new(-8, 0).unwrap()));
        b.apply_move(Move::Arranging(Position::new(8, 0).unwrap(), Position::new(7, -4).unwrap()));
        b.apply_move(Move::Planting(FlowerTile::Rhododendron, Position::new(8, 0).unwrap()));
        b.apply_move(Move::Arranging(Position::new(0, -8).unwrap(), Position::new(0, -4).unwrap()));
        b.apply_move(Move::Arranging(Position::new(-8, 0).unwrap(), Position::new(-7, 4).unwrap()));
        b.apply_move(Move::Planting(FlowerTile::Rhododendron, Position::new(-8, 0).unwrap()));
        b.apply_move(Move::Arranging(Position::new(0, 8).unwrap(), Position::new(2, 6).unwrap()));
        b
    }

    pub fn move_count(&self) -> i16 {
        self.move_count
    }

    pub fn moves_since_planting(&self) -> i16 {
        self.moves_since_planting
    }

    pub fn get_random_move(&self, grid: &Grid) -> Option<Move> {
        let player = self.next_to_move();
        let (mut reserve, mut played_tiles) = match player {
            Player::Host => (self.reserve_host.clone(), self.played_tiles_host.clone()),
            Player::Guest => (self.reserve_guest.clone(), self.played_tiles_guest.clone()),
        };
        if self.move_count < 2 {
            if self.move_count == 0 {
                loop {
                    let (tile, amount) = reserve.remove(thread_rng().gen_range(0..reserve.len()));
                    let Tile::Flower(flower) = tile;
                    if amount == 0 {
                        unreachable!("All flower tiles should be in the reserve for move one.")
                    }
                    return Some(Move::Planting(flower, Position::new(0, -8).unwrap()));
                }
            } else {
                loop {
                    let south_gate = grid.index(&Position::new(0, -8).unwrap());
                    let (tile, _) = south_gate.expect("There should be a Tile in the south gate for the second move.");
                    let Tile::Flower(flower) = tile;
                    for (t, a) in reserve {
                        if t == tile && a == 0 {
                            unreachable!("All flower tiles should be in the reserve for move two.")
                        }
                    }
                    return Some(Move::Planting(flower, Position::new(0, 8).unwrap()));
                }
            }
        }
        fn random_plant(grid: &Grid, reserve: &mut Vec<(Tile, u8)>) -> Option<Move> {
            let index = thread_rng().gen_range(0..reserve.len());
            let (Tile::Flower(t), amount) = reserve.index_mut(index);
            #[cfg(debug_assertions)]
            println!("{amount}");
            if *amount == 0 {
                reserve.remove(index);
                None
            } else {
                let mut plants: Vec<Move> = grid
                    .open_gates()
                    .iter()
                    .map(|g| Move::Planting(*t, g.clone()))
                    .collect();
                if plants.len() > 0 {
                    let m = plants.remove(thread_rng().gen_range(0..plants.len()));
                    Some(m)
                } else {
                    *reserve = Vec::new();
                    None
                }
            }
        }
        fn random_move(
            grid: &Grid,
            board: &Board,
            played_tiles: &mut Vec<(Tile, Position)>,
        ) -> Option<Move> {
            let (tile, position) =
                played_tiles.remove(thread_rng().gen_range(0..played_tiles.len()));
            let possible_moves = all_possibilities_for_piece_to_move(board, grid, tile, position);
            if possible_moves.len() == 0 {
                None
            } else {
                let m = possible_moves
                    .get(thread_rng().gen_range(0..possible_moves.len()))
                    .unwrap();
                Some(m.clone())
            }
        }
        loop {
            return if reserve.len() > 0 && played_tiles.len() > 0 {
                if thread_rng().gen_ratio(1, 3) {
                    let mo = random_plant(grid, &mut reserve);
                    if mo.is_none() {
                        continue;
                    };
                    mo
                } else {
                    let mo = random_move(grid, self, &mut played_tiles);
                    if mo.is_none() {
                        continue;
                    }
                    mo
                }
            } else if reserve.len() == 0 && played_tiles.len() > 0 {
                let mo = random_move(grid, self, &mut played_tiles);
                if mo.is_none() {
                    continue;
                }
                mo
            } else if played_tiles.len() == 0 && reserve.len() > 0 {
                let mo = random_plant(grid, &mut reserve);
                if mo.is_none() {
                    continue;
                };
                mo
            } else {
                None
            };
        }
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

    pub fn finished(
        &self,
        harmonie_list: Vec<(Owner, Position, Position)>,
        perspective: Player,
    ) -> Option<Output> {
        let mut guest_harmonies = Vec::new();
        let mut guest_crossing_harmonies = 0;
        let mut host_harmonies = Vec::new();
        let mut host_crossing_harmonies = 0;

        for harmonie in harmonie_list.into_iter() {
            let (owner, p1, p2) = harmonie;
            let ((p1_x, p1_y), (p2_x, p2_y)) = (p1.value(), p2.value());
            match owner {
                Owner::Host => {
                    if (p1_y as isize) * (p2_y as isize) < 0
                        || (p1_x as isize) * (p2_x as isize) < 0
                    {
                        host_crossing_harmonies += 1;
                    }
                    host_harmonies.push((p1, p2));
                }
                Owner::Guest => {
                    if (p1_y as isize) * (p2_y as isize) < 0
                        || (p1_x as isize) * (p2_x as isize) < 0
                    {
                        guest_crossing_harmonies += 1;
                    }
                    guest_harmonies.push((p1, p2));
                }
            }
        }

        fn finish_ring(
            harmonies: Vec<(Position, Position)>,
            ring_fragment: Vec<Position>,
        ) -> Vec<Vec<Position>> {
            if ring_fragment.len() == 0 {
                panic!("a ring_fragment need at least one element")
            }
            let mut rings = Vec::new();
            for (index, harmony) in harmonies.iter().enumerate() {
                let fragment_start = ring_fragment.first().unwrap();
                let fragment_end = ring_fragment.last().unwrap();
                let harmony_matches_rings_start = harmony.0 == *fragment_start || harmony.1 == *fragment_start;
                let harmony_matches_rings_end = harmony.0 == *fragment_end || harmony.1 == *fragment_end;
                if harmony_matches_rings_start && harmony_matches_rings_end {
                    rings.push(ring_fragment.clone());
                    continue;
                }
                let fragment_piece = if harmony_matches_rings_start {
                    fragment_start
                } else if harmony_matches_rings_end {
                    fragment_end
                } else {
                    continue;
                };
                let (mut h, mut r) = (harmonies.clone(), ring_fragment.clone());
                h.remove(index);
                if harmony.0 == *fragment_piece {
                    r.push(harmony.1.clone());
                } else {
                    r.push(harmony.0.clone());
                }
                let mut rings_found = finish_ring(h, r);
                rings.append(&mut rings_found);
            }
            rings
        }

        fn ring_contains_center(ring: Vec<Position>) -> bool {
            let mut winding_number = 0;
            for i in 0..ring.len() {
                let start =  ring[i].value();
                let end =  ring[(i+1) % ring.len()].value();
                if (start.0 == 0 && start.1 == 0) || (end.0 == 0 && end.1 == 0) ||
                    (start.0 == 0 && end.0 == 0 && start.1 * end.1 < 0) ||
                    (start.1 == 0 && end.1 == 0 && start.0 * end.0 < 0) {
                    //If the tiles are on the Origin or cross it then the cant form a Ring containing it
                    return false;
                }
                let start = (start.0 as isize, start.1 as isize);
                let end = (end.0 as isize, end.1 as isize);
                //MAGIC
                //Not really i count how often the right edge is crossed upwards / downwards
                if end.1 <= 0 {
                    if start.1 > 0 {
                        if ( (start.0 - end.0) * (0 - end.1) - (0 -  end.0) * (start.1 - end.1) ) > 0 {
                            winding_number += 1;
                        }
                    }
                } else {
                    if start.1 <= 0 {
                        if ( (start.0 - end.0) * (0 - end.1) - (0 -  end.0) * (start.1 - end.1) ) < 0 {
                            winding_number -= 1;
                        }
                    }
                }
            }
            winding_number != 0
        }

        if guest_harmonies.len() + host_harmonies.len() > 0 {
            let mut guest_won = false;
            'test_harmony: while let Some(harmony) = guest_harmonies.pop() {
                let r = vec![harmony.0, harmony.1];
                for ring in finish_ring(guest_harmonies.clone(), r) {
                    if ring_contains_center(ring) {
                        guest_won = true;
                        break 'test_harmony;
                    }
                }
            }

            let mut host_won = false;
            'test_harmony: while let Some(harmony) = host_harmonies.pop() {
                let r = vec![harmony.0, harmony.1];
                for ring in finish_ring(host_harmonies.clone(), r) {
                    if ring_contains_center(ring) {
                        host_won = true;
                        break 'test_harmony;
                    }
                }
            }

            if host_won || guest_won {
                return Some(if host_won && guest_won {
                    Output::Draw
                } else if host_won && perspective == Player::Host {
                    Output::Win
                } else if guest_won && perspective == Player::Guest {
                    Output::Win
                } else {
                    Output::Loss
                });
            }
        }

        let reserve_size_guest = self
            .reserve_guest
            .iter()
            .map(|(_, c)| *c)
            .reduce(|total, c| total + c)
            .unwrap();
        let reserve_size_host = self
            .reserve_host
            .iter()
            .map(|(_, c)| *c)
            .reduce(|total, c| total + c)
            .unwrap();

        if reserve_size_guest == 0 || reserve_size_host == 0 {
            if guest_crossing_harmonies == host_crossing_harmonies {
                return Some(Output::Draw);
            }
            return Some(match perspective {
                Player::Guest => {
                    if guest_crossing_harmonies > host_crossing_harmonies {
                        Output::Win
                    } else {
                        Output::Loss
                    }
                }
                Player::Host => {
                    if guest_crossing_harmonies < host_crossing_harmonies {
                        Output::Win
                    } else {
                        Output::Loss
                    }
                }
            });
        }

        if self.moves_since_planting >= 50 {
            return Some(Output::Draw);
        };
        return None;
    }

    pub fn all_legal_moves(&self, grid: &mut Grid) -> Moves {
        if self.finished(grid.list_all_harmonies(), self.next_to_move()).is_some() {
            return Vec::new();
        }
        let (tiles_played, reserve) = match self.next_to_move() {
            Player::Guest => (&self.played_tiles_guest, &self.reserve_guest),
            Player::Host => (&self.played_tiles_host, &self.reserve_host),
        };
        let mut move_set: Moves = Vec::with_capacity(20 * tiles_played.len());
        if self.move_count < 2 {
            if self.move_count == 0 {
                let south_gate = Position::new(0, -8).unwrap();
                for (Tile::Flower(flower), amount) in reserve {
                    if *amount > 0 {
                        move_set.push(Move::Planting(*flower, south_gate.clone()));
                    } else {
                        unreachable!("All flower tiles should be in the reserve for move one.")
                    }
                }
            } else {
                let south_gate = Position::new(0, -8).unwrap();
                let noth_gate = Position::new(0, 8).unwrap();
                let (tile, _) = grid.index(&south_gate).expect("There should be a Tile in the south gate for the second move.");
                let Tile::Flower(flower) = tile;
                for (t, a) in reserve {
                    if *t == tile && *a == 0 {
                        unreachable!("All flower tiles should be in the reserve for move two.")
                    }
                }
                return vec![Move::Planting(flower, noth_gate)];
            }
            return move_set;
        }
        for (tile, position) in tiles_played {
            let mut moves_for_piece =
                all_possibilities_for_piece_to_move(self, grid, *tile, position.clone());
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
                    for (_tile_type, position) in &mut self.played_tiles_host {
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
