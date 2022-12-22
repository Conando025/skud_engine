#![allow(unused_imports, unused_imports, dead_code)]
//mod flower_skud;
//mod hatch_boxes;
mod monte_carlo_tree_search;
mod ultimate_tic_tac_toe;
//mod skud_pai_sho;
//mod tic_tac_toe;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

//use crate::flower_skud::{FlowerTile, Position, Tile};
//pub use flower_skud::{Board, Grid, Move};
use monte_carlo_tree_search::{create_root_node, engine, trim_tree, Mode, Node, Origin, Player};
pub use ultimate_tic_tac_toe::{Board, Move};

fn main() {
    /*
    let problem = Board::create_test();
    let grid = Grid::create(&problem);
    let _ = problem.finished( grid.list_all_harmonies(),Player::Host);
    */
    //get a board
    let board = Board::empty();
    let mut root = create_root_node(board);
    loop {
        //let tree = engine(root, Mode::Iterations(1_000_000));
        let tree = engine(root, Mode::Time(Duration::from_secs(3)));
        let root_node = <Rc<RefCell<Node>> as Borrow<RefCell<Node>>>::borrow(&tree).borrow();
        let sim_count = root_node.simulations;
        let mut children = root_node.children.clone();
        children.sort_by_key(|child| (**child).borrow().simulations);
        for (index, child) in children.iter().enumerate() {
            let child_node = (**child).borrow();
            if let Origin::Parent(_, the_move) = &child_node.origin {
                println!(
                    "{index:0>3}: [ {:0>7.3} | {:0>7.3} | {:0>7.3}] {:0>7.3}% for {:?}",
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
        //return;
        println!("Enter a move index to continue with that move:");
        let index = loop {
            let mut line = String::new();
            let _bytes_read = std::io::stdin().read_line(&mut line).unwrap();
            line.pop();
            let Ok(input): Result<usize, _> = line.parse() else {
                println!("Enter a move index (between 0 and {}) to continue with that move:", children.len() - 1);
                continue;
            };
            if input >= children.len() {
                println!(
                    "Enter a move index (between 0 and {}) to continue with that move:",
                    children.len() - 1
                );
                continue;
            }
            break input;
        };
        let selected_node = children[index].clone();
        root = trim_tree(selected_node);
    }
}
