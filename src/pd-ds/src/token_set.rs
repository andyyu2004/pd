use pd_syntax::SyntaxKind;

pub struct TokenSet(u128);

impl TokenSet {
    pub const EMPTY: Self = Self(0);

    pub const fn new(kinds: &[SyntaxKind]) -> Self {
        assert!(SyntaxKind::__Last as u16 <= 127);
        let mut k = 0;
        let mut i = 0;
        while i < kinds.len() {
            k |= mask(kinds[i]);
            i += 1;
        }
        Self(k)
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn with(self, kind: SyntaxKind) -> Self {
        Self(self.0 | mask(kind))
    }

    pub const fn contains(&self, kind: SyntaxKind) -> bool {
        self.0 & mask(kind) != 0
    }
}

#[inline]
const fn mask(kind: SyntaxKind) -> u128 {
    1u128 << (kind as usize)
}
