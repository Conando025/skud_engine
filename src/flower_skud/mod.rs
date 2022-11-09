use crate::monte_carlo_tree_search::{Output, Player};
use std::collections::HashMap;
use std::ops::Div;
use std::{cell, vec};

mod other;
pub use other::*;

#[derive(Clone)]
pub struct Board {
    played_tiles_guest: Vec<(Tile, Position)>,
    played_tiles_host: Vec<(Tile, Position)>,
    reserve_host: Vec<(Tile, u8)>, //tiles and their counts
    reserve_guest: Vec<(Tile, u8)>,
    move_count: i16,
    moves_since_planting: i16,
}

#[derive(Clone)]
pub struct Grid {
    pub(self) cells: Vec<Cell>,
}

impl Grid {
    pub fn create(board: Board) -> Self {
        let mut cells: Vec<Cell> = Vec::with_capacity(289);
        cells.fill(None);
        let mut grid = Grid { cells };
        for (tile, pos) in &board.played_tiles_guest {
            *grid.index_mut(pos) = Some((*tile, Owner::Guest));
        }
        for (tile, pos) in &board.played_tiles_host {
            *grid.index_mut(pos) = Some((*tile, Owner::Host));
        }
        grid
    }
    fn index(&self, position: &Position) -> &Option<(Tile, Owner)> {
        let (x, y) = position.value();
        if x > 8 || y > 8 || x.abs() + y.abs() > 12 {
            panic!("Index out of bounds");
        } else {
            let (x, y) = (x + 8, y + 8);
            &self.cells[(x + y * 17) as usize]
        }
    }
    fn index_mut(&mut self, position: &Position) -> &mut Option<(Tile, Owner)> {
        let (x, y) = position.value();
        if x.abs() > 8 || y.abs() > 8 || x.abs() + y.abs() > 12 {
            panic!("Index out of bounds");
        } else {
            let (x, y) = (x + 8, y + 8);
            &mut self.cells[x as usize + y as usize * 17]
        }
    }
}

fn all_possibilities_for_piece_to_move(
    grid: &mut Grid,
    moving_tile_type: Tile,
    starting_position: Position,
) -> Vec<Move> {
    let move_range = match moving_tile_type {
        Tile::Flower(FlowerTile::Rose) | Tile::Flower(FlowerTile::Jasmine) => 3,
        Tile::Flower(FlowerTile::Chrysanthemum) | Tile::Flower(FlowerTile::Lily) => 4,
        Tile::Flower(FlowerTile::Rhododendron) | Tile::Flower(FlowerTile::WhiteJade) => 5,
    };

    let (position_x, position_y) = starting_position.value();

    let mut possible_positions: Vec<Position> = Vec::new();
    let mut tiles_in_range = Vec::new();

    for (index, cell) in grid.cells.iter_mut().enumerate() {
        let (cell_x, cell_y): (i8, i8) = (index.div(17) as i8 - 8, (index % 17) as i8 - 8);
        let Some(target_position) = Position::new(cell_x, cell_y) else {
            continue;
        };
        let (x_rel, y_rel) = (cell_x - position_x, cell_y - position_y);
        if x_rel.abs() + y_rel.abs() <= move_range && x_rel != 0 && y_rel != 0 {
            if let &mut Some(_) = cell {
                tiles_in_range.push(target_position);
                continue;
            }
            if cell_x.abs() + cell_y.abs() <= 6 {
                match (moving_tile_type, cell_x as i16 * cell_y as i16) {
                    (
                        Tile::Flower(FlowerTile::Rose)
                        | Tile::Flower(FlowerTile::Chrysanthemum)
                        | Tile::Flower(FlowerTile::Rhododendron),
                        product,
                    ) => {
                        if product < 0 {
                            continue;
                        }
                    }
                    (
                        Tile::Flower(FlowerTile::Jasmine)
                        | Tile::Flower(FlowerTile::Lily)
                        | Tile::Flower(FlowerTile::WhiteJade),
                        product,
                    ) => {
                        if product > 0 {
                            continue;
                        }
                    }
                };
            }
            if (cell_x, cell_y) == (8, 0)
                || (cell_x, cell_y) == (0, 8)
                || (cell_x, cell_y) == (-8, 0)
                || (cell_x, cell_y) == (0, -8)
            {
                continue;
            };
            possible_positions.push(target_position);
        }
    }

    let tile_data = grid.index(&starting_position).clone();
    *grid.index_mut(&starting_position) = None;

    possible_positions = possible_positions
        .into_iter()
        .filter(|target_position| {
            let cell = grid.index_mut(target_position);
            let cell_data = cell.clone();
            *cell = tile_data.clone();
            //check for clash-traps
            {
                let mut tile_left = None;
                let mut pos_to_check = starting_position.clone();
                while let (None, Some(p)) =
                (grid.index(&pos_to_check), pos_to_check.add(Direction::Left))
                {
                    pos_to_check = p;
                }
                if let Some((t, _)) = grid.index(&pos_to_check) {
                    tile_left = Some(*t);
                }
                if let Some(tile_left) = tile_left {
                    pos_to_check = starting_position.clone();
                    while let (None, Some(p)) =
                    (grid.index(&pos_to_check), pos_to_check.add(Direction::Right))
                    {
                        pos_to_check = p;
                    }
                    if let Some((t, _)) = grid.index(&pos_to_check) {
                        if tile_left.clashes(t) {
                            return false;
                        }
                    }
                }

                let mut tile_down = None;
                pos_to_check = starting_position.clone();
                while let (None, Some(p)) =
                (grid.index(&pos_to_check), pos_to_check.add(Direction::Down))
                {
                    pos_to_check = p;
                }
                if let Some((t, _)) = grid.index(&pos_to_check) {
                    tile_down = Some(*t);
                }
                if let Some(tile_down) = tile_down {
                    pos_to_check = starting_position.clone();
                    while let (None, Some(p)) =
                    (grid.index(&pos_to_check), pos_to_check.add(Direction::Up))
                    {
                        pos_to_check = p;
                    }
                    if let Some((t, _)) = grid.index(&pos_to_check) {
                        if tile_down.clashes(t) {
                            return false;
                        }
                    }
                }

            }

            //check for new clashes
            {
                let mut pos_to_check = target_position.clone();
                while let (None, Some(p)) =
                    (grid.index(&pos_to_check), pos_to_check.add(Direction::Down))
                {
                    pos_to_check = p;
                }
                if let Some((t, _)) = grid.index(&pos_to_check) {
                    if moving_tile_type.clashes(t) {
                        return false;
                    }
                }
                pos_to_check = target_position.clone();
                while let (None, Some(p)) =
                    (grid.index(&pos_to_check), pos_to_check.add(Direction::Up))
                {
                    pos_to_check = p;
                }
                if let Some((t, _)) = grid.index(&pos_to_check) {
                    if moving_tile_type.clashes(t) {
                        return false;
                    }
                }
                pos_to_check = target_position.clone();
                while let (None, Some(p)) =
                    (grid.index(&pos_to_check), pos_to_check.add(Direction::Left))
                {
                    pos_to_check = p;
                }
                if let Some((t, _)) = grid.index(&pos_to_check) {
                    if moving_tile_type.clashes(t) {
                        return false;
                    }
                }
                pos_to_check = target_position.clone();
                while let (None, Some(p)) = (
                    grid.index(&pos_to_check),
                    pos_to_check.add(Direction::Right),
                ) {
                    pos_to_check = p;
                }
                if let Some((t, _)) = grid.index(&pos_to_check) {
                    if moving_tile_type.clashes(t) {
                        return false;
                    }
                }
            }
            *grid.index_mut(target_position) = cell_data;
            true
        }).collect();

    *grid.index_mut(&starting_position) = tile_data.clone();

    let mut check_list: HashMap<Position, bool> = HashMap::new();

    let mut left_to_check = vec![
        (starting_position.clone(), Direction::Up, 1),
        (starting_position.clone(), Direction::Down, 1),
        (starting_position.clone(), Direction::Left, 1),
        (starting_position.clone(), Direction::Right, 1),
    ];

    'flood_fill: while let Some((p, d, c)) = left_to_check.pop() {
        if check_list.get(&p).is_some() {
            continue;
        }
        let Some(new_pos) = p.add(d) else {
            continue
        };
        let Some(true) = check_list.get(&p) else {
            continue
        };
        for occupied_position in &tiles_in_range {
            if new_pos == *occupied_position {
                check_list.insert(new_pos.clone(), false);
                continue 'flood_fill;
            }
        }
        check_list.insert(new_pos.clone(), true);
        left_to_check.push((new_pos.clone(), Direction::Up, c + 1));
        left_to_check.push((new_pos.clone(), Direction::Down, c + 1));
        left_to_check.push((new_pos.clone(), Direction::Left, c + 1));
        left_to_check.push((new_pos, Direction::Right, c + 1));
    }

    let mut legal_moves = Vec::new();
    for possible_position in possible_positions {
        if *check_list.get(&possible_position).unwrap_or(&false) {
            legal_moves.push(Move::Arranging(
                starting_position.clone(),
                possible_position,
            ));
        }
    }

    legal_moves
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

    fn guest_add_tile(&mut self, tile: Tile, position: Position) {
        self.played_tiles_guest.push((tile, position));
        for (tile_type, amount) in &mut self.reserve_guest {
            if *tile_type == tile {
                *amount -= 1;
                break;
            }
        }
    }

    pub fn finished(&self, _perspective: Player) -> Option<Output> {
        if self.moves_since_planting >= 50 {
            return Some(Output::Draw);
        };
        todo!()
    }

    pub fn all_legal_moves(&self, grid: &mut Grid) -> Moves {
        let mut move_set: Moves = Vec::new();
        match self.next_to_move() {
            Player::Guest => {
                let mut open_gates: Vec<Position> = Position::GATES.to_vec();
                for (tile, position) in &self.played_tiles_guest {
                    let mut moves_for_piece =
                        all_possibilities_for_piece_to_move(grid, *tile, position.clone());
                    move_set.append(&mut moves_for_piece);
                    open_gates = open_gates
                        .into_iter()
                        .filter(|gate| *gate == *position)
                        .collect();
                }
                for (_, position) in &self.played_tiles_host {
                    open_gates = open_gates
                        .into_iter()
                        .filter(|gate| *gate == *position)
                        .collect();
                }
                for (Tile::Flower(flower), _) in &self.reserve_guest {
                    for gate in open_gates.iter() {
                        move_set.push(Move::Planting(*flower, gate.clone()));
                    }
                }
            }
            Player::Host => {
                let mut open_gates: Vec<Position> = Position::GATES.to_vec();
                for (tile, position) in &self.played_tiles_host {
                    let mut moves_for_piece =
                        all_possibilities_for_piece_to_move(grid, *tile, position.clone());
                    move_set.append(&mut moves_for_piece);
                    open_gates = open_gates
                        .into_iter()
                        .filter(|gate| *gate == *position)
                        .collect();
                }
                for (_, position) in &self.played_tiles_guest {
                    open_gates = open_gates
                        .into_iter()
                        .filter(|gate| *gate == *position)
                        .collect();
                }
                for (Tile::Flower(flower), _) in &self.reserve_host {
                    for gate in open_gates.iter() {
                        move_set.push(Move::Planting(*flower, gate.clone()));
                    }
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
                        for (tile, count) in self.reserve_host.iter_mut() {
                            if *tile == Tile::Flower(flower_tile) {
                                *count -= 1;
                                break;
                            }
                        }
                    }
                    Player::Guest => {
                        self.guest_add_tile(Tile::Flower(flower_tile), position);
                        for (tile, count) in self.reserve_guest.iter_mut() {
                            if *tile == Tile::Flower(flower_tile) {
                                *count -= 1;
                                break;
                            }
                        }
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
