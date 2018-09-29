use std::ops::{Range, RangeFrom, RangeInclusive, RangeToInclusive};

pub enum ExpectedCalls {
    Any,
    Exact(u64),
    AtLeast(u64),
    AtMost(u64),
    Between { start: u64, end: u64 },
    BetweenInclusive { start: u64, end: u64 },
}

impl ExpectedCalls {
    pub(crate) fn matches_value(&self, value: u64) -> bool {
        match self {
            ExpectedCalls::Any => true,
            ExpectedCalls::Exact(expected) => *expected == value,
            ExpectedCalls::AtLeast(min) => value >= *min,
            ExpectedCalls::AtMost(max) => value <= *max,
            ExpectedCalls::Between { start, end } => value >= *start && value < *end,
            ExpectedCalls::BetweenInclusive { .. } => unimplemented!(),
        }
    }
}

impl From<u64> for ExpectedCalls {
    fn from(value: u64) -> ExpectedCalls {
        ExpectedCalls::Exact(value)
    }
}

impl From<RangeFrom<u64>> for ExpectedCalls {
    fn from(range: RangeFrom<u64>) -> ExpectedCalls {
        ExpectedCalls::AtLeast(range.start)
    }
}

impl From<Range<u64>> for ExpectedCalls {
    fn from(range: Range<u64>) -> ExpectedCalls {
        ExpectedCalls::Between {
            start: range.start,
            end: range.end,
        }
    }
}

impl From<RangeInclusive<u64>> for ExpectedCalls {
    fn from(range: RangeInclusive<u64>) -> ExpectedCalls {
        let (start, end) = range.into_inner();

        ExpectedCalls::BetweenInclusive { start, end }
    }
}

impl From<RangeToInclusive<u64>> for ExpectedCalls {
    fn from(range: RangeToInclusive<u64>) -> ExpectedCalls {
        ExpectedCalls::AtMost(range.end)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn any_matches_any_value() {
        assert!(ExpectedCalls::Any.matches_value(0));
        assert!(ExpectedCalls::Any.matches_value(1));
        assert!(ExpectedCalls::Any.matches_value(23));
        assert!(ExpectedCalls::Any.matches_value(100));
    }

    #[test]
    fn exact_matches_specified_value() {
        assert!(ExpectedCalls::Exact(4).matches_value(4));
    }

    #[test]
    fn exact_does_not_match_other_values() {
        assert!(!ExpectedCalls::Exact(6).matches_value(1));
        assert!(!ExpectedCalls::Exact(6).matches_value(0));
        assert!(!ExpectedCalls::Exact(10).matches_value(1));
        assert!(!ExpectedCalls::Exact(100).matches_value(1));
    }

    #[test]
    fn at_least_matches_minimum_value() {
        assert!(ExpectedCalls::AtLeast(4).matches_value(4));
    }

    #[test]
    fn at_least_matches_values_above_minimum() {
        assert!(ExpectedCalls::AtLeast(4).matches_value(5));
        assert!(ExpectedCalls::AtLeast(4).matches_value(10));
        assert!(ExpectedCalls::AtLeast(4).matches_value(100));
    }

    #[test]
    fn at_least_does_not_match_values_below_minimum() {
        assert!(!ExpectedCalls::AtLeast(50).matches_value(49));
        assert!(!ExpectedCalls::AtLeast(50).matches_value(0));
        assert!(!ExpectedCalls::AtLeast(50).matches_value(16));
    }

    #[test]
    fn at_most_matches_maximum() {
        assert!(ExpectedCalls::AtMost(40).matches_value(40));
    }

    #[test]
    fn at_most_matches_values_below_maximum() {
        assert!(ExpectedCalls::AtMost(40).matches_value(0));
        assert!(ExpectedCalls::AtMost(40).matches_value(39));
        assert!(ExpectedCalls::AtMost(40).matches_value(24));
    }

    #[test]
    fn at_most_does_not_match_values_above_maximum() {
        assert!(!ExpectedCalls::AtMost(20).matches_value(21));
        assert!(!ExpectedCalls::AtMost(20).matches_value(67));
        assert!(!ExpectedCalls::AtMost(20).matches_value(100));
    }

    #[test]
    fn between_does_not_match_values_outside_of_range() {
        assert!(!ExpectedCalls::Between { start: 10, end: 20 }.matches_value(0));
        assert!(!ExpectedCalls::Between { start: 10, end: 20 }.matches_value(9));
        assert!(!ExpectedCalls::Between { start: 10, end: 20 }.matches_value(21));
        assert!(!ExpectedCalls::Between { start: 10, end: 20 }.matches_value(40));
    }

    #[test]
    fn between_does_not_match_end_value() {
        assert!(!ExpectedCalls::Between { start: 1, end: 3 }.matches_value(3));
    }

    #[test]
    fn between_matches_start_value() {
        assert!(ExpectedCalls::Between { start: 1, end: 3 }.matches_value(1));
    }

    #[test]
    fn between_matches_values_in_range() {
        assert!(ExpectedCalls::Between { start: 10, end: 20 }.matches_value(11));
        assert!(ExpectedCalls::Between { start: 10, end: 20 }.matches_value(15));
        assert!(ExpectedCalls::Between { start: 10, end: 20 }.matches_value(19));
    }
}
