use crate::monte_carlo_tree_search::{Output, Player};
use std::ops::Div;

#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    X,
    O,
}

impl Into<Player> for Piece {
    fn into(self) -> Player {
        match self {
            Piece::X => Player::Guest,
            Piece::O => Player::Host,
        }
    }
}

impl From<Player> for Piece {
    fn from(player: Player) -> Self {
        match player {
            Player::Host => Piece::O,
            Player::Guest => Piece::X,
        }
    }
}

type Cell = Option<Piece>;

#[derive(Clone)]
pub struct Board {
    cells: [Option<Piece>; 9],
    next_to_move: Piece,
}

fn winning_three(a: Cell, b: Cell, c: Cell) -> bool {
    a.is_some() && a == b && b == c
}

impl Board {
    pub fn empty() -> Self {
        Board {
            cells: [
                None,
                None,
                None,
                None,
                Some(Piece::O),
                None,
                None,
                None,
                None,
            ],
            next_to_move: Piece::X,
        }
    }

    pub fn finished(&self, player: Player) -> Option<Output> {
        if winning_three(self.cells[0], self.cells[4], self.cells[8]) {
            return if self.cells[4].unwrap() == player.into() {
                Some(Output::Win)
            } else {
                Some(Output::Loss)
            };
        }
        if winning_three(self.cells[2], self.cells[4], self.cells[6]) {
            return if self.cells[4].unwrap() == player.into() {
                Some(Output::Win)
            } else {
                Some(Output::Loss)
            };
        }
        for i in 0..3 {
            if winning_three(
                self.cells[0 + 3 * i],
                self.cells[1 + 3 * i],
                self.cells[2 + 3 * i],
            ) {
                return if self.cells[0 + 3 * i].unwrap() == player.into() {
                    Some(Output::Win)
                } else {
                    Some(Output::Loss)
                };
            }
            if winning_three(self.cells[0 + i], self.cells[3 + i], self.cells[6 + i]) {
                return if self.cells[i].unwrap() == player.into() {
                    Some(Output::Win)
                } else {
                    Some(Output::Loss)
                };
            }
        }
        for cell in self.cells {
            if cell.is_none() {
                return None;
            }
        }
        Some(Output::Draw)
    }

    pub fn all_legal_moves(&self) -> Vec<Move> {
        let mut list = Vec::new();
        for (index, cell) in self.cells.iter().enumerate() {
            if cell.is_none() {
                list.push(Move::Place(index % 3, index.div(3)))
            }
        }
        list
    }

    pub fn apply_move(&mut self, a_move: Move) {
        let Move::Place(x, y) = a_move;
        self.cells[x + 3 * y] = Some(self.next_to_move);
        self.next_to_move = match self.next_to_move {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        };
    }

    pub fn next_to_move(&self) -> Player {
        self.next_to_move.into()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Place(usize, usize),
}
