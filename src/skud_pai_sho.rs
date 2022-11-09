use crate::monte_carlo_tree_search::Output;

#[derive(Clone)]
pub struct Board {
    played_tiles_guest: Vec<(Tile, Position)>,
    played_tiles_host: Vec<(Tile, Position)>,
    reserve_host: Vec<(Tile, u8)>, //tiles and their counts
    reserve_guest: Vec<(Tile, u8)>,
    move_count: i16,
}

#[derive(Clone)]
enum Owner {
    Host,
    Guest,
}

type Cell = Option<(Tile, Owner)>;

struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    fn create(board: &Board) -> Self {
        let mut cells: Vec<Cell> = Vec::with_capacity(289);
        cells.fill(None);
        let mut grid = Grid { cells };
        for (tile, pos) in &board.played_tiles_guest {
            *grid.index_mut(pos.clone()) = Some((*tile, Owner::Guest));
        }
        for (tile, pos) in &board.played_tiles_host {
            *grid.index_mut(pos.clone()) = Some((*tile, Owner::Host));
        }
        grid
    }
    fn index(&self, position: Position) -> &Option<(Tile, Owner)> {
        let Position { x, y } = position;
        if x > 8 || y > 8 || x.abs() + y.abs() > 12 {
            panic!("Index out of bounds");
        } else {
            let (x, y) = (x + 8, y + 8);
            &self.cells[(x + y * 17) as usize]
        }
    }
    fn index_mut(&mut self, position: Position) -> &mut Option<(Tile, Owner)> {
        let Position { x, y } = position;
        if x > 8 || y > 8 || x.abs() + y.abs() > 12 {
            panic!("Index out of bounds");
        } else {
            let (x, y) = (x + 8, y + 8);
            &mut self.cells[(x + y * 17) as usize]
        }
    }
}

impl Board {
    fn guest_add_tile(&mut self, tile: Tile, position: Position) {
        self.played_tiles_guest.push((tile, position));
        for (tile_type, amount) in &mut self.reserve_guest {
            if *tile_type == tile {
                *amount -= 1;
                break;
            }
        }
    }

    pub fn finished(&self) -> Option<Output> {
        todo!()
    }

    pub fn all_legal_moves(&self) -> Moves {
        #[allow(unused_mut)]
        let mut move_set: Moves = Vec::new();
        if self.move_count % 2 == 0 {
            for (_, _position) in &self.played_tiles_guest {
                todo!()
            }
            for (tile, _) in &self.reserve_guest {
                if let Tile::Flower(_flower_tile) = tile {
                    todo!()
                }
            }
        } else {
            for (_, _position) in &self.played_tiles_host {
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
        for (tile_type, amount) in &mut self.reserve_host {
            if *tile_type == tile {
                *amount -= 1;
                break;
            }
        }
    }

    pub fn apply_move(&mut self, a_move: Move) {
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
                            HarmonyBonus::PlaceAccentTile(accent_tile_move, position) => {
                                match accent_tile_move {
                                    AccentTileMove::Boat(pushed_position) => {
                                        for (_, tile_position) in &mut self.played_tiles_guest {
                                            if *tile_position == position {
                                                *tile_position = pushed_position;
                                                break;
                                            }
                                        }
                                        self.host_add_tile(
                                            Tile::Accent(AccentTile::Boat),
                                            position,
                                        );
                                    }
                                    AccentTileMove::Wrapper(accent_tile) => {
                                        self.host_add_tile(
                                            Tile::Accent(accent_tile),
                                            position.clone(),
                                        );
                                        match accent_tile {
                                            AccentTile::Wheel => {
                                                for (_, tile_position) in
                                                    &mut self.played_tiles_guest
                                                {
                                                    Self::wheel_a_tile(
                                                        position.clone(),
                                                        tile_position,
                                                    );
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
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
                        match &bonus_move {
                            HarmonyBonus::PlaceAccentTile(accent_tile_move, position) => {
                                match accent_tile_move {
                                    AccentTileMove::Boat(pushed_position) => {
                                        for (_, tile_position) in &mut self.played_tiles_host {
                                            if *tile_position == *position {
                                                *tile_position = pushed_position.clone();
                                                break;
                                            }
                                        }
                                        self.host_add_tile(
                                            Tile::Accent(AccentTile::Boat),
                                            position.clone(),
                                        );
                                    }
                                    AccentTileMove::Wrapper(accent_tile) => {
                                        self.host_add_tile(
                                            Tile::Accent(*accent_tile),
                                            position.clone(),
                                        );
                                        match accent_tile {
                                            AccentTile::Wheel => {
                                                for (_, tile_position) in
                                                    &mut self.played_tiles_host
                                                {
                                                    Self::wheel_a_tile(
                                                        position.clone(),
                                                        tile_position,
                                                    );
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            HarmonyBonus::PlantFlower(flower_tile, position) => {
                                self.host_add_tile(Tile::Flower(*flower_tile), position.clone());
                            }
                            HarmonyBonus::PlantSpecialFlower(special_flower_tile, position) => {
                                self.host_add_tile(
                                    Tile::Special(*special_flower_tile),
                                    position.clone(),
                                );
                            }
                        }
                    }
                }
            }
        }
        self.move_count += 1;
    }

    fn wheel_a_tile(position: Position, tile_position: &mut Position) {
        if tile_position.y + 1 == position.y
            && (tile_position.x == position.x || tile_position.x - 1 == position.x)
        {
            tile_position.x += 1;
            return;
        }
        if tile_position.y - 1 == position.y
            && (tile_position.x == position.x || tile_position.x + 1 == position.x)
        {
            tile_position.x -= 1;
            return;
        }
        if tile_position.x + 1 == position.x
            && (tile_position.y == position.y || tile_position.y + 1 == position.y)
        {
            tile_position.x += 1;
            return;
        }
        if tile_position.x - 1 == position.x
            && (tile_position.y == position.y || tile_position.y - 1 == position.y)
        {
            tile_position.x -= 1;
            return;
        }
    }
}

#[derive(Clone)]
pub enum Move {
    Planting(FlowerTile, Position),
    Arranging(Position, Position, Option<HarmonyBonus>),
}

pub type Moves = Vec<Move>;

#[derive(Clone)]
pub enum HarmonyBonus {
    PlantFlower(FlowerTile, Position),
    PlantSpecialFlower(SpecialFlowerTile, Position),
    PlaceAccentTile(AccentTileMove, Position),
}

#[derive(Clone)]
pub enum AccentTileMove {
    Boat(Position),
    Wrapper(AccentTile),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    x: i8,
    y: i8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Flower(FlowerTile),
    Special(SpecialFlowerTile),
    Accent(AccentTile),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FlowerTile {
    Rose,
    Chrysanthemum,
    Rhododendron,
    Jasmine,
    Lily,
    WhiteJade,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpecialFlowerTile {
    Lotus,
    Orchid,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AccentTile {
    Rock,
    Wheel,
    Knotweed,
    Boat,
}
