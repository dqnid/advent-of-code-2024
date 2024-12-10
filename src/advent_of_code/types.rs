use std::collections::HashSet;

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

// Bridge repair
pub type CalibrationResult = u128;
pub type CalibrationEquation = Vec<CalibrationResult>;
pub struct Calibration(pub CalibrationResult, pub CalibrationEquation);

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    SUM,
    MUL,
    COMB,
}

// Resonant collinearity
pub type RoofRow = Vec<Option<char>>;
pub type RoofMap = Vec<RoofRow>;

#[derive(Debug, PartialEq)]
pub struct Antena(pub usize, pub usize, pub char); //y,x,freq
pub type AntenaList = Vec<Antena>;

pub type AntinodeCount = usize;
pub type Antinode = (usize, usize);
pub type AntinodeList = HashSet<Antinode>;
