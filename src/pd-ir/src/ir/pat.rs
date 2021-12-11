use pd_syntax::ast;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Pat {
    // TODO this can probably become an expr
    Literal(),
}
