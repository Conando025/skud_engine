use crate::monte_carlo_tree_search::{Output, Player};
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Board {
    sub_boards: [SubBoard; 9],
    forced_sub_board: Option<usize>,
    next: Player,
}

impl Board {
    pub fn empty() -> Self {
        /*
        Board {
            sub_boards: [
                SubBoard {
                    state: State::NotFinished,
                    cells: [
                        Cell::X,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::X,
                        Cell::Empty,
                        Cell::O,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                    ],
                },
                SubBoard {
                    state: State::NotFinished,
                    cells: [
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::O,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                    ],
                },
                SubBoard {
                    state: State::NotFinished,
                    cells: [
                        Cell::O,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                    ],
                },
                SubBoard {
                    state: State::NotFinished,
                    cells: [
                        Cell::Empty,
                        Cell::Empty,
                        Cell::X,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                    ],
                },
                SubBoard {
                    state: State::NotFinished,
                    cells: [
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::X,
                        Cell::Empty,
                        Cell::O,
                        Cell::Empty,
                        Cell::O,
                    ],
                },
                SubBoard {
                    state: State::Win(Player::Guest),
                    cells: [
                        Cell::Empty,
                        Cell::X,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::X,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::X,
                        Cell::Empty,
                    ],
                },
                SubBoard {
                    state: State::NotFinished,
                    cells: [
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::X,
                        Cell::O,
                        Cell::Empty,
                    ],
                },
                SubBoard {
                    state: State::NotFinished,
                    cells: [
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::O,
                        Cell::Empty,
                        Cell::O,
                        Cell::Empty,
                        Cell::X,
                        Cell::Empty,
                    ],
                },
                SubBoard {
                    state: State::NotFinished,
                    cells: [
                        Cell::O,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::Empty,
                        Cell::X,
                    ],
                },
            ],
            forced_sub_board: Some(3),
            next: Player::Host,
        }
        */
        Board {
            sub_boards: [
                SubBoard::empty(),
                SubBoard::empty(),
                SubBoard::empty(),
                SubBoard::empty(),
                SubBoard::empty(),
                SubBoard::empty(),
                SubBoard::empty(),
                SubBoard::empty(),
                SubBoard::empty(),
            ],
            forced_sub_board: None,
            next: Player::Guest,
        }
    }
    pub fn finished(&self, player: Player) -> Option<Output> {
        if three_winning_boards(
            self.sub_boards[0].state,
            self.sub_boards[4].state,
            self.sub_boards[8].state,
        ) {
            match self.sub_boards[4].state {
                State::Win(p) => {
                    return Some(if p == player {
                        Output::Win
                    } else {
                        Output::Loss
                    })
                }
                _ => {
                    unreachable!()
                }
            }
        }
        if three_winning_boards(
            self.sub_boards[2].state,
            self.sub_boards[4].state,
            self.sub_boards[6].state,
        ) {
            match self.sub_boards[4].state {
                State::Win(p) => {
                    return Some(if p == player {
                        Output::Win
                    } else {
                        Output::Loss
                    })
                }
                _ => {
                    unreachable!()
                }
            }
        }
        for i in 0..3 {
            if three_winning_boards(
                self.sub_boards[0 + 3 * i].state,
                self.sub_boards[1 + 3 * i].state,
                self.sub_boards[2 + 3 * i].state,
            ) {
                match self.sub_boards[3 * i].state {
                    State::Win(p) => {
                        return Some(if p == player {
                            Output::Win
                        } else {
                            Output::Loss
                        })
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            if three_winning_boards(
                self.sub_boards[0 + i].state,
                self.sub_boards[3 + i].state,
                self.sub_boards[6 + i].state,
            ) {
                match self.sub_boards[i].state {
                    State::Win(p) => {
                        return Some(if p == player {
                            Output::Win
                        } else {
                            Output::Loss
                        })
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
        }
        for sub_board in self.sub_boards.iter() {
            if sub_board.state == State::NotFinished {
                return None;
            }
        }
        Some(Output::Draw)
    }

    pub fn all_legal_moves(&self) -> Vec<Move> {
        if self.finished(self.next).is_some() {
            return Vec::new();
        }
        return if let Some(index) = self.forced_sub_board {
            let mut prime_moves = self.sub_boards[index].all_legal_moves();
            prime_moves
                .iter_mut()
                .map(|Move { x, y }| Move {
                    x: *x + (index % 3) * 3,
                    y: *y + (index / 3) * 3,
                })
                .collect()
        } else {
            let mut moves = Vec::with_capacity(81);
            for (index, sub) in self.sub_boards.iter().enumerate() {
                if sub.state == State::NotFinished {
                    let mut prime_moves = self.sub_boards[index].all_legal_moves();
                    prime_moves = prime_moves
                        .iter_mut()
                        .map(|Move { x, y }| Move {
                            x: *x + (index % 3) * 3,
                            y: *y + (index / 3) * 3,
                        })
                        .collect();
                    moves.append(&mut prime_moves)
                }
            }
            moves
        };
    }

    pub fn apply_move(&mut self, m: Move) {
        let c = match self.next {
            Player::Host => Cell::O,
            Player::Guest => Cell::X,
        };
        let index = (m.x / 3) + (m.y / 3) * 3;
        let sub = &mut self.sub_boards[index];
        let i = m.x % 3 + (m.y % 3) * 3;
        sub.apply_move(i, c);
        self.forced_sub_board = if self.sub_boards[i].state == State::NotFinished {
            Some(i)
        } else {
            None
        };
        self.next = match self.next {
            Player::Host => Player::Guest,
            Player::Guest => Player::Host,
        }
    }

    pub fn next_to_move(&self) -> Player {
        self.next
    }

    pub fn get_random_move(&self) -> Option<Move> {
        let mut all_moves = self.all_legal_moves();
        Some(all_moves.remove(thread_rng().gen_range(0..all_moves.len())))
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..9 {
            for x in 0..9 {
                write!(f, "|")?;
                let i1 = x / 3 + (y / 3) * 3;
                let i2 = x % 3 + (y % 3) * 3;
                match self.sub_boards[i1].cells[i2] {
                    Cell::Empty => write!(f, " ")?,
                    Cell::O => write!(f, "o")?,
                    Cell::X => write!(f, "x")?,
                }
            }
            write!(f, "|\n")?;
        }
        std::fmt::Result::Ok(())
    }
}

#[derive(Clone)]
struct SubBoard {
    state: State,
    cells: [Cell; 9],
}

impl SubBoard {
    const fn empty() -> Self {
        SubBoard {
            state: State::NotFinished,
            cells: [Cell::Empty; 9],
        }
    }

    fn all_legal_moves(&self) -> Vec<Move> {
        let mut list = Vec::with_capacity(9);
        for (index, &cell) in self.cells.iter().enumerate() {
            if cell == Cell::Empty {
                list.push(Move {
                    x: index % 3,
                    y: index / 3,
                })
            }
        }
        list
    }

    fn apply_move(&mut self, i: usize, c: Cell) {
        self.cells[i] = c;
        self.update();
    }

    fn update(&mut self) {
        if three_winning_cells(self.cells[0], self.cells[4], self.cells[8]) {
            match self.cells[4] {
                Cell::X => self.state = State::Win(Player::Guest),
                Cell::O => self.state = State::Win(Player::Host),
                Cell::Empty => unreachable!(),
            };
            return;
        }
        if three_winning_cells(self.cells[2], self.cells[4], self.cells[6]) {
            match self.cells[4] {
                Cell::X => self.state = State::Win(Player::Guest),
                Cell::O => self.state = State::Win(Player::Host),
                Cell::Empty => unreachable!(),
            };
            return;
        }
        for i in 0..3 {
            if three_winning_cells(
                self.cells[0 + 3 * i],
                self.cells[1 + 3 * i],
                self.cells[2 + 3 * i],
            ) {
                match self.cells[3 * i] {
                    Cell::X => self.state = State::Win(Player::Guest),
                    Cell::O => self.state = State::Win(Player::Host),
                    Cell::Empty => unreachable!(),
                };
                return;
            }
            if three_winning_cells(self.cells[0 + i], self.cells[3 + i], self.cells[6 + i]) {
                match self.cells[i] {
                    Cell::X => self.state = State::Win(Player::Guest),
                    Cell::O => self.state = State::Win(Player::Host),
                    Cell::Empty => unreachable!(),
                };
                return;
            }
        }
        for cell in self.cells {
            if cell == Cell::Empty {
                return;
            }
        }
        self.state = State::Draw;
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Cell {
    Empty,
    X,
    O,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum State {
    NotFinished,
    Draw,
    Win(Player),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Move {
    x: usize,
    y: usize,
}

fn three_winning_cells(a: Cell, b: Cell, c: Cell) -> bool {
    a != Cell::Empty && a == b && b == c
}

fn three_winning_boards(a: State, b: State, c: State) -> bool {
    match (a, b, c) {
        (State::Win(pa), State::Win(pb), State::Win(pc)) => pa == pb && pb == pc,
        _ => false,
    }
}
