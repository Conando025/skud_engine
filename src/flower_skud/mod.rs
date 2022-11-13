use crate::monte_carlo_tree_search::{Output, Player};
use std::borrow::Borrow;
use std::collections::{HashMap, VecDeque};
use std::ops::Div;
use std::vec;

mod other;
pub use other::*;
mod board;
pub use board::*;
mod grid;
pub use grid::*;

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

    let mut check_list: HashMap<Position, bool> = HashMap::new();

    let mut left_to_check: VecDeque<(Position, Direction, u8)> = vec![
        (starting_position.clone(), Direction::Up, 1),
        (starting_position.clone(), Direction::Down, 1),
        (starting_position.clone(), Direction::Left, 1),
        (starting_position.clone(), Direction::Right, 1),
    ]
    .into();

    //flood fill
    while let Some((p, d, c)) = left_to_check.pop_front() {
        let Some(new_pos) = p.add(d) else {
            continue;
        };
        if let Some(true) = check_list.get(&new_pos) {
            continue;
        };
        if let Some(_t) = grid.index(&new_pos) {
            check_list.insert(new_pos.clone(), false);
            continue;
        }
        check_list.insert(new_pos.clone(), true);
        if c < move_range {
            left_to_check.push_back((new_pos.clone(), Direction::Up, c + 1));
            left_to_check.push_back((new_pos.clone(), Direction::Down, c + 1));
            left_to_check.push_back((new_pos.clone(), Direction::Left, c + 1));
            left_to_check.push_back((new_pos.clone(), Direction::Right, c + 1));
        }
    }

    let mut legal_moves = Vec::new();
    for (possible_position, reachable) in check_list.into_iter() {
        if reachable
            && is_landable_for(possible_position.clone(), moving_tile_type)
            && !creates_a_clash(
                grid,
                moving_tile_type,
                starting_position.clone(),
                possible_position.clone(),
            )
        {
            legal_moves.push(Move::Arranging(
                starting_position.clone(),
                possible_position.clone(),
            ));
        }
    }

    legal_moves
}

fn creates_a_clash(
    grid: &mut Grid,
    moving_tile_type: Tile,
    starting_position: Position,
    end_position: Position,
) -> bool {
    for d in Direction::ALL {
        let mut move_in_d = Some(end_position.clone());
        while move_in_d.is_some() && grid.index(&move_in_d.clone().unwrap()).is_none() {
            move_in_d = move_in_d.unwrap().add(d);
        }
        if move_in_d.is_none() {
            continue;
        }
        let move_in_d_pos = move_in_d.unwrap();
        if move_in_d_pos.is_gate() {
            continue;
        }
        let Some((next_tile_in_direction, _owner)) = grid.index(&move_in_d_pos) else {
            continue;
        };

        if next_tile_in_direction.clashes(&moving_tile_type) {
            return true;
        }
    }
    //todo("Clash traps")
    false
}

fn is_landable_for(p: Position, t: Tile) -> bool {
    if p.is_gate() {
        return false;
    }
    let (x, y) = p.value();
    //check gardens
    if x.abs() + y.abs() <= 6 {
        match t {
            Tile::Flower(FlowerTile::Rose)
            | Tile::Flower(FlowerTile::Chrysanthemum)
            | Tile::Flower(FlowerTile::Rhododendron) => {
                if 0 > x as isize * y as isize {
                    return false;
                }
            }
            Tile::Flower(FlowerTile::Jasmine)
            | Tile::Flower(FlowerTile::Lily)
            | Tile::Flower(FlowerTile::WhiteJade) => {
                if x as isize * y as isize > 0 {
                    return false;
                }
            }
        }
    }

    true
}
