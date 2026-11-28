use oxigraph::model::Triple;
use oxigraph::sparql::QuerySolution;

/// One streamed result of an Oxigraph store.
#[derive(Debug)]
pub enum ResultItem {
    /// A `SELECT` solution.
    Solution(QuerySolution),
    /// The answer of an `ASK` query.
    Boolean(bool),
    /// A `CONSTRUCT` or `DESCRIBE` triple.
    Triple(Triple),
}
