use crate::monte_carlo_tree_search::{Output, Player};
use std::vec;

mod other;
pub use other::*;
mod board;
pub use board::*;
mod grid;
pub use grid::*;

fn print_checklist(check_list: &[bool; 289]) {
    print!("    ");
    for column in -8..=8 {
        print!("  {:2}  ", column);
    }
    print!("\n");
    for row in (0..17).rev() {
        print!("{:<2}: ", row as isize - 8);
        for column in 0..17 {
            let v = if check_list[column + row * 17] {"t"} else {"f"};
            print!("[  {v} ]");
        }
        print!("\n");
    }
}

pub fn all_possibilities_for_piece_to_move(
    board: &Board,
    grid: &Grid,
    moving_tile_type: Tile,
    starting_position: Position,
) -> Vec<Move> {
    let check_list = flood_fill(grid, moving_tile_type, &starting_position);



    let mut possible_clash_coords = Vec::with_capacity(6);
    for (t, p) in board
        .played_tiles_host
        .iter()
        .chain(board.played_tiles_guest.iter())
    {
        if moving_tile_type.clashes(t) && !p.is_gate() {
            possible_clash_coords.push(p.value());
        }
    }

    let mut legal_moves = Vec::with_capacity(50);
    'check_position: for (index, reachable) in check_list.into_iter().enumerate() {
        if !reachable {
            continue 'check_position;
        }
        let possible_position =
            Position::new((index % 17) as i8 - 8, (index / 17) as i8 - 8).unwrap();
        if correct_garden(possible_position.clone(), moving_tile_type) {
            let (x, y) = possible_position.value();
            for possible_clash_coord in &possible_clash_coords {
                if x == possible_clash_coord.0 {
                    if y > possible_clash_coord.1 {
                        if grid.next_tile_in_direction(&possible_position, Direction::Down).unwrap().0.clashes(&moving_tile_type) {
                            continue 'check_position;
                        }
                    } else if y < possible_clash_coord.1 {
                        if grid.next_tile_in_direction(&possible_position, Direction::Up).unwrap().0.clashes(&moving_tile_type) {
                            continue 'check_position;
                        }
                    }
                }
                if y == possible_clash_coord.1 {
                    if x > possible_clash_coord.0 {
                        if grid.next_tile_in_direction(&possible_position, Direction::Left).unwrap().0.clashes(&moving_tile_type) {
                            continue 'check_position;
                        }
                    } else if x < possible_clash_coord.0 {
                        if grid.next_tile_in_direction(&possible_position, Direction::Right).unwrap().0.clashes(&moving_tile_type) {
                            continue 'check_position;
                        }
                    }
                }
            }
            legal_moves.push(Move::Arranging(
                starting_position.clone(),
                possible_position.clone(),
            ));
        }
    }

    legal_moves
}

fn flood_fill(grid: &Grid, moving_tile_type: Tile, starting_position: &Position) -> [bool; 289] {
    let move_range = match moving_tile_type {
        Tile::Flower(FlowerTile::Rose) | Tile::Flower(FlowerTile::Jasmine) => 3,
        Tile::Flower(FlowerTile::Chrysanthemum) | Tile::Flower(FlowerTile::Lily) => 4,
        Tile::Flower(FlowerTile::Rhododendron) | Tile::Flower(FlowerTile::WhiteJade) => 5,
    };

    let mut check_list: [bool; 289] = [false; 289];

    let mut tiles_in_direction: [Option<Tile>; 4] = [None; 4];
    let mut pos_in_direction: [Option<Position>; 4] = [None, None, None, None];
    for (i, d) in Direction::ALL.into_iter().enumerate() {
        if let Some((t, p, _)) = grid.next_tile_in_direction(starting_position, d) {
            if !p.is_gate() {
                tiles_in_direction[i] = Some(t);
                pos_in_direction[i] = Some(p);
            }
        }
    }

    let mut directions_to_check: Vec<Direction> = Vec::with_capacity(4);
    if let (Some(up), Some(down)) = (&tiles_in_direction[0], &tiles_in_direction[1]) {
        if !up.clashes(down) {
            directions_to_check.push(Direction::Left);
            directions_to_check.push(Direction::Right);
        }
    } else {
        directions_to_check.push(Direction::Left);
        directions_to_check.push(Direction::Right);
    };
    if let (Some(left), Some(right)) = (&tiles_in_direction[2], &tiles_in_direction[3]) {
        if !left.clashes(right) {
            directions_to_check.push(Direction::Up);
            directions_to_check.push(Direction::Down);
        }
    } else {
        directions_to_check.push(Direction::Up);
        directions_to_check.push(Direction::Down);
    };

    let mut left_to_check: Vec<(Position, Direction)> =
        Vec::with_capacity(2 * move_range * (move_range + 1));
    left_to_check.append(
        &mut directions_to_check
            .iter()
            .map(|&d| (starting_position.clone(), d))
            .collect(),
    );

    //flood fill
    for _ in 0..move_range {
        let move_: Vec<(Position, Direction)> = left_to_check.drain(0..).collect();
        for (p, d) in move_ {
            let Some(new_pos) = p.add(d) else {
                continue;
            };
            if new_pos.is_gate() {
                continue;
            }
            let (n_x, n_y) = new_pos.value();
            let (n_x, n_y) = ((n_x + 8) as usize, (n_y + 8) as usize);
            if check_list[n_x + n_y * 17] {
                continue
            };
            if let Some(_t) = grid.index(&new_pos) {
                continue;
            }
            check_list[n_x + n_y * 17] = true;
            left_to_check.append(
                &mut directions_to_check
                    .iter()
                    .map(|&d| (new_pos.clone(), d))
                    .collect(),
            );
        };
    }
    check_list
}

fn correct_garden(p: Position, t: Tile) -> bool {
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
