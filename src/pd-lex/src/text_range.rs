use super::*;
use std::ops::{Bound, Index, IndexMut, Range, RangeBounds};

#[derive(Hash, Default, Copy, Clone, Eq, PartialEq)]
pub struct TextRange {
    // Invariant: start <= end
    start: usize,
    end: usize,
}

impl TextRange {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self { start, end }
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

impl fmt::Debug for TextRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start(), self.end())
    }
}

impl RangeBounds<usize> for TextRange {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Excluded(&self.end)
    }
}

impl<T> From<TextRange> for Range<T>
where
    T: From<usize>,
{
    #[inline]
    fn from(r: TextRange) -> Self {
        r.start().into()..r.end().into()
    }
}

impl Index<TextRange> for str {
    type Output = str;

    #[inline]
    fn index(&self, index: TextRange) -> &str {
        &self[Range::<usize>::from(index)]
    }
}

impl Index<TextRange> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: TextRange) -> &str {
        &self[Range::<usize>::from(index)]
    }
}

impl IndexMut<TextRange> for str {
    #[inline]
    fn index_mut(&mut self, index: TextRange) -> &mut str {
        &mut self[Range::<usize>::from(index)]
    }
}

impl IndexMut<TextRange> for String {
    #[inline]
    fn index_mut(&mut self, index: TextRange) -> &mut str {
        &mut self[Range::<usize>::from(index)]
    }
}
