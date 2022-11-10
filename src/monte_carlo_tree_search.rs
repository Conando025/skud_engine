use crate::flower_skud::Grid;
use crate::{Board, Move};
use rand::{thread_rng, Rng};
use std::borrow::Borrow;
use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

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

pub fn engine(board: Board, iterations: usize) -> CellNodeReference {
    let root = Node {
        simulations: 0,
        win_count: 0,
        draw_count: 0,
        possible_moves: board.all_legal_moves(&mut Grid::create(board.clone())),
        children: Vec::new(),
        origin: Origin::Root(board),
    };
    let root = Rc::new(RefCell::new(root));
    for _iteration in 0..iterations {
        algorithm(root.clone());
    }
    return root;
}

fn algorithm(root: CellNodeReference) {
    let (board, node_ref) = match selection_phase(root) {
        NodeType::Leaf(leaf_node) => expansion_phase(leaf_node),
        NodeType::End(node) => (extract_board(node.clone()), node),
    };
    let simulation_value = simulation_phase(board);
    backpropagation(simulation_value, &Rc::downgrade(&node_ref));
}

enum NodeType {
    Leaf(CellNodeReference),
    End(CellNodeReference),
}

fn selection_phase(boxed_node: CellNodeReference) -> NodeType {
    let node = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&boxed_node).borrow();
    if node.possible_moves.len() > 0 {
        NodeType::Leaf(boxed_node.clone())
    } else {
        if node.children.len() == 0 {
            NodeType::End(boxed_node.clone())
        } else {
            let mut node_to_explore: CellNodeReference = node.children[0].clone();
            let mut max_alpha = 0.0;
            for child in &node.children {
                let child_node =
                    <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&child).borrow();
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

fn expansion_phase(leaf_node_reference: CellNodeReference) -> (Board, CellNodeReference) {
    let mut board = extract_board(leaf_node_reference.clone());
    let mut leaf_node =
        <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&leaf_node_reference).borrow_mut();
    let possible_next_moves = &mut leaf_node.possible_moves;
    let next_move =
        possible_next_moves.remove(thread_rng().gen_range(0..possible_next_moves.len()));
    board.apply_move(next_move.clone());
    let new_node = Rc::new(RefCell::new(Node {
        simulations: 0,
        win_count: 0,
        draw_count: 0,
        possible_moves: board.all_legal_moves(&mut Grid::create(board.clone())),
        children: Vec::new(),
        origin: Origin::Parent(Rc::downgrade(&leaf_node_reference), next_move.clone()),
    }));
    leaf_node.children.push(new_node.clone());
    (board, new_node)
}

fn backpropagation(value: Output, node: &Weak<RefCell<Node>>) {
    if let Some(bar) = node.upgrade() {
        let mut node_content: RefMut<Node> =
            <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&bar).borrow_mut();
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
        Player::Host => Player::Guest,
        Player::Guest => Player::Host,
    };
    let mut grid = Grid::create(board.clone());
    let mut possible_next_moves = board.all_legal_moves(&mut grid);
    while possible_next_moves.len() > 0 {
        let next_move = possible_next_moves
            .get(thread_rng().gen_range(0..possible_next_moves.len()))
            .unwrap();
        grid.apply_move(next_move.clone(), board.next_to_move());
        board.apply_move(next_move.clone());
        #[cfg(debug_assertions)]
        println!("{:?}", grid);
        possible_next_moves = board.all_legal_moves(&mut grid);
    }
    board.finished(player).unwrap_or(Output::Draw) //the or is for petty draws
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
