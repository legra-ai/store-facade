use core::fmt;

/// An RDF term in its N-Triples (RDF 1.2) form, such as `<urn:a>`,
/// `"01"^^<http://www.w3.org/2001/XMLSchema#integer>`, `"x"@en--ltr` or
/// `<<( <urn:a> <urn:b> <urn:c> )>>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TermText(String);

impl TermText {
    /// Wraps a term the binding rendered as N-Triples.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }

    /// The N-Triples text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TermText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A SPARQL variable name, without its `?`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariableName(String);

impl VariableName {
    /// Wraps a variable name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// The name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One solution: its bound variables and their terms.
///
/// bounded: contract queries project at most a handful of variables.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SolutionObservation(Vec<(VariableName, TermText)>);

impl SolutionObservation {
    /// An empty solution.
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Adds a binding.
    #[must_use]
    pub fn bind(mut self, variable: VariableName, term: TermText) -> Self {
        self.0.push((variable, term));
        self
    }

    /// The term bound to `variable`.
    #[must_use]
    pub fn get(&self, variable: &str) -> Option<&TermText> {
        self.0
            .iter()
            .find(|(name, _)| name.as_str() == variable)
            .map(|(_, term)| term)
    }

    /// The number of bound variables.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether no variable is bound.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// A result item as the contract suite sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Observation {
    /// An `ASK` answer.
    Boolean(bool),
    /// A `SELECT` solution.
    Solution(SolutionObservation),
    /// A `CONSTRUCT` or `DESCRIBE` triple.
    Triple(TermText, TermText, TermText),
}

impl Observation {
    /// The answer, if this is a boolean.
    #[must_use]
    pub const fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(answer) => Some(*answer),
            _ => None,
        }
    }

    /// The solution, if this is one.
    #[must_use]
    pub const fn as_solution(&self) -> Option<&SolutionObservation> {
        match self {
            Self::Solution(solution) => Some(solution),
            _ => None,
        }
    }
}
