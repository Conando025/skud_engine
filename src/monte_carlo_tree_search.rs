use crate::flower_skud::{Board, Grid, Move};
use rand::{thread_rng, Rng};
use std::borrow::Borrow;
use std::cell::{RefCell, RefMut};
use std::ops::Add;
use std::rc::{Rc, Weak};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub type CellNodeReference = Rc<RefCell<Node>>;

type Moves = Vec<Move>;

pub struct Node {
    pub simulations: u32,
    pub win_count: u32,
    pub draw_count: u32,
    possible_moves: Moves,
    pub children: Vec<CellNodeReference>,
    pub origin: Origin,
}

pub enum Origin {
    Parent(Weak<RefCell<Node>>, Move),
    Root(Board),
}

pub fn create_root_node(board: Board) -> CellNodeReference {
    let root = Node {
        simulations: 0,
        win_count: 0,
        draw_count: 0,
        possible_moves: board.all_legal_moves(&mut Grid::create(board.clone())),
        children: Vec::new(),
        origin: Origin::Root(board),
    };
    Rc::new(RefCell::new(root))
}

pub fn trim_tree(node: CellNodeReference) -> CellNodeReference {
    let board = extract_board(node.clone());
    let mut node_contents = (*node).borrow_mut();
    node_contents.origin = Origin::Root(board);
    drop(node_contents);
    node
}

pub fn engine(root: CellNodeReference, mode: Mode) -> CellNodeReference {
    println!("{}", Grid::create(extract_board(root.clone())));
    match mode {
        Mode::Iterations(iterations) => {
            for _iteration in 0..iterations {
                algorithm(root.clone());
            }
        }
        Mode::Time(duration) => {
            let stop_time = Instant::now().add(duration);
            while Instant::now() < stop_time {
                algorithm(root.clone());
            }
        }
    }
    return root;
}

#[cfg(debug_assertions)]
const MULTI_COUNT: usize = 1;
#[cfg(not(debug_assertions))]
const MULTI_COUNT: usize = 5;

fn algorithm(root: CellNodeReference) {
    match selection_phase(root) {
        NodeType::Leaf(leaf_node) => {
            let nodes = expansion_phase(leaf_node);
            let (tx, rx) = mpsc::channel();
            let (board_list, node_list): (Vec<(usize, Board)>, Vec<CellNodeReference>) = nodes
                .into_iter()
                .enumerate()
                .map(|(i, (b, n))| ((i, b), n))
                .unzip();
            for (index, board) in board_list {
                let tx = tx.clone();
                thread::spawn(move || {
                    let simulation_value = simulation_phase(board);
                    tx.send((simulation_value, index)).unwrap();
                });
            }
            drop(tx);
            for (outcome, node_index) in rx {
                backpropagation(outcome, &Rc::downgrade(&node_list[node_index]));
            }
        }
        NodeType::End(node) => {
            let board = extract_board(node.clone());
            let outcome = simulation_phase(board);
            backpropagation(outcome, &Rc::downgrade(&node));
        }
    };
}

enum NodeType {
    Leaf(CellNodeReference),
    End(CellNodeReference),
}

fn selection_phase(boxed_node: CellNodeReference) -> NodeType {
    let node = (*boxed_node).borrow();
    if node.possible_moves.len() > 0 {
        NodeType::Leaf(boxed_node.clone())
    } else {
        if node.children.len() == 0 {
            NodeType::End(boxed_node.clone())
        } else {
            let mut node_to_explore: CellNodeReference = node.children[0].clone();
            let mut max_alpha = 0.0;
            for child in &node.children {
                let child_node = (**child).borrow();
                let win_rate: f64 = (child_node.win_count as f64 * 2.0
                    + child_node.draw_count as f64 * 1.0)
                    / (child_node.simulations as f64 * 2.0);
                let alpha: f64 = win_rate
                    + (2.0 * (node.simulations as f64).ln() / child_node.simulations as f64).sqrt();
                if alpha > max_alpha {
                    max_alpha = alpha;
                    node_to_explore = child.clone();
                }
            }
            selection_phase(node_to_explore)
        }
    }
}

fn extract_board(node: CellNodeReference) -> Board {
    let content = (*node).borrow();
    match &content.origin {
        Origin::Root(board_ref) => board_ref.clone(),
        Origin::Parent(parent, node_move) => {
            let mut board = extract_board(parent.upgrade().unwrap());
            board.apply_move(node_move.clone());
            board
        }
    }
}

fn expansion_phase(leaf_node_reference: CellNodeReference) -> Vec<(Board, CellNodeReference)> {
    let board = extract_board(leaf_node_reference.clone());
    let mut leaf_node = (*leaf_node_reference).borrow_mut();
    let possible_next_moves = &mut leaf_node.possible_moves;

    let mut node_list = Vec::with_capacity(MULTI_COUNT);
    for _ in 0..MULTI_COUNT {
        if possible_next_moves.len() == 0 {
            break;
        };
        let next_move =
            possible_next_moves.remove(thread_rng().gen_range(0..possible_next_moves.len()));
        let mut new_node_board = board.clone();
        new_node_board.apply_move(next_move.clone());
        let new_node = Rc::new(RefCell::new(Node {
            simulations: 0,
            win_count: 0,
            draw_count: 0,
            possible_moves: new_node_board.all_legal_moves(&mut Grid::create(new_node_board.clone())),
            children: Vec::new(),
            origin: Origin::Parent(Rc::downgrade(&leaf_node_reference), next_move.clone()),
        }));
        node_list.push((new_node_board, new_node));
    }
    for (_b, node) in node_list.iter() {
        leaf_node.children.push(node.clone());
    }
    node_list
}

fn backpropagation(value: Output, node: &Weak<RefCell<Node>>) {
    if let Some(bar) = node.upgrade() {
        let mut node_content: RefMut<Node> = (*bar).borrow_mut();
        node_content.simulations += 1;
        let value = match value {
            Output::Win => {
                node_content.win_count += 1;
                Output::Loss
            }
            Output::Draw => {
                node_content.draw_count += 1;
                Output::Draw
            }
            Output::Loss => Output::Win,
        };
        if let Origin::Parent(parent, _node_move) = node_content.origin.borrow() {
            backpropagation(value, parent)
        }
    }
}

fn simulation_phase(mut board: Board) -> Output {
    let player = match board.next_to_move() {
        Player::Host => Player::Host,
        Player::Guest => Player::Guest,
    };
    let mut grid = Grid::create(board.clone());
    let mut harmony_list = grid.list_all_harmonies();
    let outcome = loop {
        let board_state = board.finished(harmony_list.clone(), player);
        if board_state.is_some() {
            break board_state;
        } else {
            let Some(next_move) = board.get_random_move(&grid) else {
                break board_state;
            };
            #[cfg(debug_assertions)]
            {
                println!("{next_move:?}");
                println!("{grid}");
            }
            grid.apply_move(next_move.clone(), board.next_to_move(), &mut harmony_list);
            board.apply_move(next_move.clone());
            #[cfg(debug_assertions)]
            println!("{grid}");
        }
    };
    outcome.unwrap_or(Output::Draw) //the or is for petty draws
}

#[derive(PartialEq, Copy, Clone)]
pub enum Player {
    Host,
    Guest,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Output {
    Win,
    Draw,
    Loss,
}

pub enum Mode {
    Iterations(usize),
    Time(Duration),
}
