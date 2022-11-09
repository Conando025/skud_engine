#![allow(unreachable_code, dead_code)]

mod flower_skud;
mod hatch_boxes;
mod monte_carlo_tree_search;
mod skud_pai_sho;
mod tic_tac_toe;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub use flower_skud::{Board, Move};
use monte_carlo_tree_search::{engine, Node, Origin};

fn main() {
    //get a board
    #[allow(unused_variables)]
    let board = Board::empty();
    //run evaluation
    let root = engine(board, 1_000_000);
    let root_node = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&root).borrow();
    let sim_count = root_node.simulations;
    for child in root_node.children.iter() {
        let child_node = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&child).borrow();
        if let Origin::Parent(_, the_move) = &child_node.origin {
            println!(
                "{:?}: [ {:>6.3} | {:>6.3} | {:>6.3}]  {:>7.3}%",
                the_move,
                child_node.win_count as f64 / child_node.simulations as f64 * 100.0,
                child_node.draw_count as f64 / child_node.simulations as f64 * 100.0,
                (child_node.simulations - child_node.win_count - child_node.draw_count) as f64
                    / child_node.simulations as f64
                    * 100.0,
                child_node.simulations as f64 / sim_count as f64 * 100.0
            );
        }
    }
}
