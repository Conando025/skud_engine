use crate::{Board, Move, Moves};
use rand::{thread_rng, Rng};
use std::borrow::Borrow;
use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

pub type CellNodeReference = Rc<RefCell<Node>>;

pub struct Node {
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

pub fn engine(board: Board) -> Moves {
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
    let leaf_node = selection_phase(root);
    expansion_phase(leaf_node);
}

fn selection_phase(boxed_node: CellNodeReference) -> CellNodeReference {
    let node = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&boxed_node).borrow();
    if node.possible_moves.len() > 0 {
        boxed_node.clone()
    } else {
        let mut node_to_explore: CellNodeReference = node.children[0].clone();
        let mut max_alpha = 0.0;
        for child in &node.children {
            let child_node = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&child).borrow();
            let win_rate: f64 = (child_node.win_count as f64 / 2.0) / child_node.simulations as f64;
            let alpha: f64 = win_rate
                + 2_f64.sqrt()
                    * ((node.simulations as f64).ln() / child_node.simulations as f64).sqrt();
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
        Origin::Root(board_ref) => board_ref.clone(),
        Origin::Parent(parent, node_move) => {
            let mut board = extract_board(parent.upgrade().unwrap());
            board.apply_move(node_move.clone());
            board
        }
    }
}

fn expansion_phase(leaf_node_reference: CellNodeReference) {
    let mut board = extract_board(leaf_node_reference.clone());
    let mut possible_next_moves = board.all_legal_moves();
    let next_move =
        possible_next_moves.remove(thread_rng().gen_range(0..=possible_next_moves.len()));
    board.apply_move(next_move.clone());
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
        origin: Origin::Parent(Rc::downgrade(&leaf_node_reference), next_move.clone()),
    }));
    leaf_node_reference.borrow_mut().children.push(new_node);
    backpropagation(simulation_value, &Rc::downgrade(&leaf_node_reference));
}

fn backpropagation(value: i8, node: &Weak<RefCell<Node>>) {
    if let Some(bar) = node.upgrade() {
        let mut node_content: RefMut<Node> =
            <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&bar).borrow_mut();
        node_content.win_count += value as i32;
        node_content.simulations += 1;
        if let Origin::Parent(parent, _node_move) = node_content.origin.borrow() {
            backpropagation(value, parent)
        }
    }
}

fn simulation_phase(mut board: Board) -> Output {
    while let None = board.finished() {
        let mut possible_next_moves = board.all_legal_moves();
        let next_move =
            possible_next_moves.remove(thread_rng().gen_range(0..=possible_next_moves.len()));
        board.apply_move(next_move);
    }
    board.finished().unwrap()
}

pub enum Output {
    Win,
    Loss,
    Draw,
}
