#[derive(Clone)]
pub enum Owner {
    Host,
    Guest,
}

pub type Cell = Option<(Tile, Owner)>;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Tile {
    pub fn clashes(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Tile::Flower(FlowerTile::Rose), &Tile::Flower(FlowerTile::Jasmine)) => true,
            (&Tile::Flower(FlowerTile::Chrysanthemum), &Tile::Flower(FlowerTile::Lily)) => true,
            (&Tile::Flower(FlowerTile::Rhododendron), &Tile::Flower(FlowerTile::WhiteJade)) => true,
            (&Tile::Flower(FlowerTile::Jasmine), &Tile::Flower(FlowerTile::Rose)) => true,
            (&Tile::Flower(FlowerTile::Lily), &Tile::Flower(FlowerTile::Chrysanthemum)) => true,
            (&Tile::Flower(FlowerTile::WhiteJade), &Tile::Flower(FlowerTile::Rhododendron)) => true,
            _ => false,
        }
    }

    pub fn harmonizes(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Tile::Flower(FlowerTile::Rose), &Tile::Flower(FlowerTile::Chrysanthemum)) => true,
            (&Tile::Flower(FlowerTile::Chrysanthemum), &Tile::Flower(FlowerTile::Rhododendron)) => true,
            (&Tile::Flower(FlowerTile::Rhododendron), &Tile::Flower(FlowerTile::Jasmine)) => true,
            (&Tile::Flower(FlowerTile::Jasmine), &Tile::Flower(FlowerTile::Lily)) => true,
            (&Tile::Flower(FlowerTile::Lily), &Tile::Flower(FlowerTile::WhiteJade)) => true,
            (&Tile::Flower(FlowerTile::WhiteJade), &Tile::Flower(FlowerTile::Rose)) => true,
            (&Tile::Flower(FlowerTile::Chrysanthemum), &Tile::Flower(FlowerTile::Rose)) => true,
            (&Tile::Flower(FlowerTile::Rhododendron), &Tile::Flower(FlowerTile::Chrysanthemum)) => true,
            (&Tile::Flower(FlowerTile::Jasmine), &Tile::Flower(FlowerTile::Rhododendron)) => true,
            (&Tile::Flower(FlowerTile::Lily), &Tile::Flower(FlowerTile::Jasmine)) => true,
            (&Tile::Flower(FlowerTile::WhiteJade), &Tile::Flower(FlowerTile::Lily)) => true,
            (&Tile::Flower(FlowerTile::Rose), &Tile::Flower(FlowerTile::WhiteJade)) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Move {
    Planting(FlowerTile, Position),
    Arranging(Position, Position),
}

pub type Moves = Vec<Move>;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    x: i8,
    y: i8,
}

impl Position {
    pub const GATES: [Position; 4] = [
        Position { x: 8, y: 0 },
        Position { x: -8, y: 0 },
        Position { x: 0, y: 8 },
        Position { x: 0, y: -8 },
    ];

    const fn out_of_bound(x: i8, y: i8) -> bool {
        x.abs() > 8 || y.abs() > 8 || x.abs() + y.abs() > 12
    }

    pub const fn new(x: i8, y: i8) -> Option<Self> {
        if Self::out_of_bound(x, y) {
            None
        } else {
            Some(Position { x, y })
        }
    }

    pub fn value(&self) -> (i8, i8) {
        (self.x, self.y)
    }

    pub fn add(&self, d: Direction) -> Option<Self> {
        match d {
            Direction::Up => Position::new(self.x, self.y + 1),
            Direction::Down => Position::new(self.x, self.y - 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Flower(FlowerTile),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FlowerTile {
    Rose,
    Chrysanthemum,
    Rhododendron,
    Jasmine,
    Lily,
    WhiteJade,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Fraction {
    nominator: isize,
    denominator: usize,
}

impl Fraction {
    pub fn new(nominator: isize, denominator: usize) -> Self {
        Fraction{nominator, denominator}
    }

    const NULL:Self = Fraction{nominator:0 , denominator: 0};
}

impl std::ops::Add for Fraction {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut nominator = self.denominator as isize * rhs.nominator + self.nominator * rhs.denominator as isize;
        let mut denominator = self.denominator * rhs.denominator;
        let k = denominator % nominator;
        if k==0 {
            nominator /=k;
            denominator /=k;
        }
        Fraction{nominator,denominator}
    }
}