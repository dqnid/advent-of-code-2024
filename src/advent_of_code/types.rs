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
