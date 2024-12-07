// Historian Hysteria - Day 1
pub type Id = i32;
pub type Key = Id;
pub type Similarity = Id;

// Red-Nosed Reports - Day 2
pub type Level = i32;
pub type Report = Vec<Level>;
pub type ReportSafety = u32;

#[derive(Debug, PartialEq, Eq)]
pub enum ReportDirection {
    Increasing,
    Decreasing,
}

// Mull It Over
pub type MulNumber = i32;

// Ceres Search
pub type XMASCount = i32;

// Print Queue
pub struct Rule(pub u32, pub u32);
pub type Queue = Vec<u32>;

// Guard Gallivant
#[derive(Clone, Copy)]
pub enum Direction {
    N,
    S,
    E,
    W,
}
pub enum Floor {
    Clear,
    Obstacle,
}
pub type PositionCount = usize;
pub type IsGuardOut = bool;
pub type FloorMap = Vec<Vec<Floor>>;
#[derive(Clone, Copy)]
pub struct Guard {
    pub x: usize,
    pub y: usize,
    pub dir: Direction,
}
