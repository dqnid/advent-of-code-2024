pub type Id = i32;
pub type Key = Id;
pub type Similarity = Id;

pub type Level = i32;
pub type Report = Vec<Level>;
pub type ReportSafety = u32;

#[derive(Debug, PartialEq, Eq)]
pub enum ReportDirection {
    Increasing,
    Decreasing,
}
