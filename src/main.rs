#![allow(unreachable_code)]

use std::borrow::Borrow;
use rand::{thread_rng, Rng};
use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

fn main() {
    println!("Hello, world!");
    //get a board
    let board = todo!();
    //run evaluation
    engine(board);
    //return evaluation
}

type CellNodeReference = Rc<RefCell<Node>>;

struct Node {
    simulations: i32,
    win_count: i32,
    possible_moves: Moves,
    children: Vec<CellNodeReference>,
    origin: Origin,
}

enum Origin {
    Parent(Weak<RefCell<Node>>, Move),
    Root(Board),
}

fn engine(board: Board) -> Moves {
    let root = Node {
        simulations: 0,
        win_count: 0, //twice as big due to draws being counted as 0.5
        possible_moves: board.all_legal_moves(),
        children: Vec::new(),
        origin: Origin::Root(board),
    };
    let root = Rc::new(RefCell::new(root));
    algorithm(root);
    todo!()
}

fn algorithm(root: CellNodeReference) {
    let mut leaf_node = selection_phase(root);
    expansion_phase(leaf_node);
}

fn selection_phase(boxed_node: CellNodeReference) -> CellNodeReference {
    let node = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&boxed_node).borrow();
    if node.possible_moves.len() > 0 {
        boxed_node.clone()
    } else {
        let mut node_to_explore:CellNodeReference = node.children[0].clone();
        let mut max_alpha = 0.0;
        for child in &node.children {
            let child_node = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&child).borrow();
            let win_rate: f64 = (child_node.win_count as f64 / 2.0) / child_node.simulations as f64;
            let alpha: f64 = win_rate + 2_f64.sqrt() * ((node.simulations as f64).ln() / child_node.simulations as f64).sqrt();
            if alpha > max_alpha {
                max_alpha = alpha;
                node_to_explore = child.clone();
            }
        }
        selection_phase(node_to_explore)
    }
}

fn extract_board(node: CellNodeReference) -> Board {
    let content = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&node).borrow();
    match &content.origin {
        Origin::Root(board_ref) => {
            board_ref.clone()
        }
        Origin::Parent(parent, node_move) => {
            let mut board = extract_board(parent.upgrade().unwrap());
            board.apply_move(*node_move);
            board
        }
    }
}

fn expansion_phase(leaf_node_reference: CellNodeReference) {

    let mut board = extract_board(leaf_node_reference.clone());
    let mut possible_next_moves = board.all_legal_moves();
    let next_move =
        possible_next_moves.remove(thread_rng().gen_range(0..=possible_next_moves.len()));
    board.apply_move(next_move);
    let simulation_value = match simulation_phase(board) {
        Output::Win => 2,
        Output::Draw => 1,
        Output::Loss => 0,
    };
    let new_node = Rc::new(RefCell::new(Node {
        simulations: 0,
        win_count: 0, //twice as big due to draws being counted as 0.5
        possible_moves: possible_next_moves,
        children: Vec::new(),
        origin: Origin::Parent(Rc::downgrade(&leaf_node_reference), next_move),
    }));
    leaf_node_reference
        .borrow_mut()
        .children
        .push(new_node);
    backpropagation(simulation_value, &Rc::downgrade(&leaf_node_reference));
}

fn backpropagation(value: i8, node: &Weak<RefCell<Node>>) {
    if let Some(bar) = node.upgrade() {
        let mut node_content: RefMut<Node> = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&bar).borrow_mut();
        node_content.win_count += value as i32;
        node_content.simulations += 1;
        if let Origin::Parent(parent, _node_move) = node_content.origin.borrow() {
            backpropagation(value, parent)
        }
    }
}

fn simulation_phase(board: Board) -> Output {
    Output::Draw
}

enum Output {
    Win,
    Loss,
    Draw,
}

type Moves = Vec<Move>;

#[derive(Clone)]
struct Board {
    played_tiles_guest: Vec<(Tile, Position)>,
    played_tiles_host: Vec<(Tile, Position)>,
    reserve_host: Vec<(Tile, u8)>, //tiles and their counts
    reserve_guest: Vec<(Tile, u8)>,
    move_count: i16,
}

impl Board {
    fn guest_add_tile(&mut self, tile: Tile, position: Position) {
        self.played_tiles_guest.push((tile, position));
        for (tile_type, mut amount) in &self.reserve_guest {
            if *tile_type == tile {
                amount -= 1;
                break;
            }
        }
    }

    fn all_legal_moves(&self) -> Moves {
        #[allow(unused_mut)]
        let mut move_set: Moves = Vec::new();
        if self.move_count % 2 == 0 {
            for (_, position) in &self.played_tiles_guest {
                todo!()
            }
            for (tile, _) in &self.reserve_guest {
                if let Tile::Flower(_flower_tile) = tile {
                    todo!()
                }
            }
        } else {
            for (_, position) in &self.played_tiles_host {
                todo!()
            }
            for (tile, _) in &self.reserve_host {
                if let Tile::Flower(_flower_tile) = *tile {
                    todo!()
                }
            }
        }
        move_set
    }

    fn host_add_tile(&mut self, tile: Tile, position: Position) {
        self.played_tiles_host.push((tile, position));
        for (tile_type, mut amount) in &self.reserve_host {
            if *tile_type == tile {
                amount -= 1;
                break;
            }
        }
    }

    fn apply_move(&mut self, a_move: Move){
        match a_move {
            Move::Planting(flower_tile, position) => {
                if self.move_count % 2 == 0 {
                    //Turn Guest
                    self.guest_add_tile(Tile::Flower(flower_tile), position);
                } else {
                    //Turn Host
                    self.host_add_tile(Tile::Flower(flower_tile), position);
                }
            }
            Move::Arranging(start, end, harmony_bonus) => {
                if self.move_count % 2 == 0 {
                    //Turn Guest
                    for (_tile_type, position) in &mut self.played_tiles_guest {
                        if *position == start {
                            *position = end;
                            break;
                        }
                    }
                    if let Some(bonus_move) = harmony_bonus {
                        match bonus_move {
                            HarmonyBonus::PlaceAccentTile(accent_tile, position) => {
                                self.guest_add_tile(Tile::Accent(accent_tile), position);
                            }
                            HarmonyBonus::PlantFlower(flower_tile, position) => {
                                self.guest_add_tile(Tile::Flower(flower_tile), position);
                            }
                            HarmonyBonus::PlantSpecialFlower(special_flower_tile, position) => {
                                self.guest_add_tile(Tile::Special(special_flower_tile), position);
                            }
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
                    if let Some(bonus_move) = harmony_bonus {
                        match bonus_move {
                            HarmonyBonus::PlaceAccentTile(accent_tile, position) => {
                                self.host_add_tile(Tile::Accent(accent_tile), position);
                            }
                            HarmonyBonus::PlantFlower(flower_tile, position) => {
                                self.host_add_tile(Tile::Flower(flower_tile), position);
                            }
                            HarmonyBonus::PlantSpecialFlower(special_flower_tile, position) => {
                                self.host_add_tile(Tile::Special(special_flower_tile), position);
                            }
                        }
                    }
                }
            }
        }
        self.move_count += 1;
    }
}

#[derive(Clone, Copy)]
enum Move {
    Planting(FlowerTile, Position),
    Arranging(Position, Position, Option<HarmonyBonus>),
}

#[derive(Clone, Copy)]
enum HarmonyBonus {
    PlantFlower(FlowerTile, Position),
    PlantSpecialFlower(SpecialFlowerTile, Position),
    PlaceAccentTile(AccentTile, Position),
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Flower(FlowerTile),
    Special(SpecialFlowerTile),
    Accent(AccentTile),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FlowerTile {
    Rose,
    Chrysanthemum,
    Rhododendron,
    Jasmine,
    Lily,
    WhiteJade,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpecialFlowerTile {
    Lotus,
    Orchid,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AccentTile {
    Rock,
    Wheel,
    Knotweed,
    Boat,
}
