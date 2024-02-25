use std::fmt;
use std::ops;

/// Represents the range A..B but has conversion methods
/// for ranges in ``std::ops`` such that ``..B`` and ``A..`` and
/// even ``..`` can be represented.
/// The boundary values A and B are inclusive.
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Range {
    /// represents all integers x satisfying ``A <= x <= B`` specified by (A, B)
    IndexIndex(usize, usize),
    /// represents all integers x satisfying ``A <= x`` specified by A
    IndexOpen(usize),
    /// represents all integers x satisfying ``x <= B`` specified by B
    OpenIndex(usize),
    /// represents all integers
    OpenOpen,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Range::IndexIndex(start, end) => if start == end {
                write!(f, "exactly {start}")
            } else {
                write!(f, "between {start} and {end}")
            },
            Range::IndexOpen(start) => write!(f, "at least {start}"),
            Range::OpenIndex(end) => write!(f, "at most {end}"),
            Range::OpenOpen => write!(f, "an arbitrary number of"),
        }
    }
}

impl From<usize> for Range {
    fn from(value: usize) -> Self {
        Range::IndexIndex(value, value)
    }
}

impl From<ops::Range<usize>> for Range {
    fn from(value: ops::Range<usize>) -> Self {
        Range::IndexIndex(value.start, value.end - 1)
    }
}

impl From<ops::RangeFrom<usize>> for Range {
    fn from(value: ops::RangeFrom<usize>) -> Self {
        Range::IndexOpen(value.start)
    }
}

impl From<ops::RangeTo<usize>> for Range {
    fn from(value: ops::RangeTo<usize>) -> Self {
        Range::OpenIndex(value.end - 1)
    }
}

impl From<ops::RangeFull> for Range {
    fn from(_value: ops::RangeFull) -> Self {
        Range::OpenOpen
    }
}

impl From<ops::RangeToInclusive<usize>> for Range {
    fn from(value: ops::RangeToInclusive<usize>) -> Self {
        Range::OpenIndex(value.end)
    }
}