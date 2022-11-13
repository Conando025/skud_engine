#![allow(unreachable_code, dead_code)]

mod flower_skud;
mod hatch_boxes;
mod monte_carlo_tree_search;
mod skud_pai_sho;
mod tic_tac_toe;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub use flower_skud::{Grid, Board, Move};
use monte_carlo_tree_search::{engine, Node, Origin};
use crate::monte_carlo_tree_search::Mode;

fn main() {
    //get a board
    #[allow(unused_variables)]
    let board = Board::empty();
    println!("{}", Grid::create(board.clone()));
    //run evaluation
    let root = engine(board, Mode::Time(Duration::from_secs(60)));
    let root_node = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&root).borrow();
    let sim_count = root_node.simulations;
    for child in root_node.children.iter() {
        let child_node = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&child).borrow();
        if let Origin::Parent(_, the_move) = &child_node.origin {
            println!(
                "[ {:0>7.3} | {:0>7.3} | {:0>7.3}] {:0>7.3}% for {:?}",
                child_node.win_count as f64 / child_node.simulations as f64 * 100.0,
                child_node.draw_count as f64 / child_node.simulations as f64 * 100.0,
                (child_node.simulations - child_node.win_count - child_node.draw_count) as f64
                    / child_node.simulations as f64
                    * 100.0,
                child_node.simulations as f64 / sim_count as f64 * 100.0,
                the_move
            );
        }
    }
    println!("We did {} simulations", sim_count);
}
