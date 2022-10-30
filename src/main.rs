fn main() {
    println!("Hello, world!");
    //get a board
    let board = todo!();
    //run evaluation
    let good_moves = engine(board);
    //return evaluation
}

fn engine(board: Board) -> Moves {
    todo!();
}

type Moves = Vec<Move> ;

struct Board {
    played_tiles: Vec<(dyn Tile, position)>,
    reserve: Vec<(dyn Tile, u8)> //tiles and their counts
}

enum Move {
    planting(FlowerTile, position),
    arranging(position, position, Option<HarmonyBonus>),
}

enum HarmonyBonus {
    plantFlower(FlowerTile, position),
    plantSpecialFlower(SpecialFlowerTile, position),
    placeAccentTile(AccentTile, position)
}

struct position {
    x: u8,
    y: u8,
}

trait Tile {

}

enum FlowerTile {
    Rose,
    Chrysanthemum,
    Rhododendron,
    Jasmine,
    Lily,
    WhiteJade,
}

enum SpecialFlowerTile {
    Lotus,
    Orchid,
}

enum AccentTile {
    Rock,
    Wheel,
    Knotweed,
    Boat,
}

impl Tile for FlowerTile {}
impl Tile for SpecialFlowerTile {}
impl Tile for AccentTile {}