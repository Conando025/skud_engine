use crate::monte_carlo_tree_search::{Output, Player};
use std::collections::HashMap;
use std::ops::Div;
use std::{cell, vec};

mod other;
pub use other::*;
mod board;
pub use board::*;

#[derive(Clone)]
pub struct Grid {
    pub(self) cells: Vec<Cell>,
}

impl Grid {
    pub fn create(board: Board) -> Self {
        let mut grid = Grid { cells: vec![None; 289] };
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
            &mut self.cells[dbg!(x as usize + y as usize * 17)]
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