#![allow(unreachable_code, dead_code)]

mod monte_carlo_tree_search;
mod skud_pai_sho;

use monte_carlo_tree_search::engine;
pub use skud_pai_sho::*;

fn main() {
    println!("Hello, world!");
    //get a board
    #[allow(unused_variables)]
    let board = todo!();
    //run evaluation
    engine(board);
    //return evaluation
}
