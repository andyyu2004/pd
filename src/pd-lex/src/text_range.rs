use super::*;
use std::ops::{Bound, Index, IndexMut, Range, RangeBounds};

#[derive(Hash, Default, Copy, Clone, Eq, PartialEq)]
pub struct Span {
    // Invariant: start <= end
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub fn zero_sized(offset: usize) -> Self {
        Self::at(offset, 0)
    }

    pub fn at(offset: usize, len: usize) -> Self {
        Self::new(offset, offset + len)
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start(), self.end())
    }
}

impl RangeBounds<usize> for Span {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Excluded(&self.end)
    }
}

impl<T> From<Span> for Range<T>
where
    T: From<usize>,
{
    #[inline]
    fn from(r: Span) -> Self {
        r.start().into()..r.end().into()
    }
}

impl Index<Span> for str {
    type Output = str;

    #[inline]
    fn index(&self, index: Span) -> &str {
        &self[Range::<usize>::from(index)]
    }
}

impl Index<Span> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: Span) -> &str {
        &self[Range::<usize>::from(index)]
    }
}

impl IndexMut<Span> for str {
    #[inline]
    fn index_mut(&mut self, index: Span) -> &mut str {
        &mut self[Range::<usize>::from(index)]
    }
}

impl IndexMut<Span> for String {
    #[inline]
    fn index_mut(&mut self, index: Span) -> &mut str {
        &mut self[Range::<usize>::from(index)]
    }
}
