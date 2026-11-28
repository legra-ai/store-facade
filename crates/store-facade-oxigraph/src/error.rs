use oxigraph::sparql::{
    QueryEvaluationError,
    SparqlSyntaxError,
    UpdateEvaluationError,
};
use oxigraph::store::{
    LoaderError,
    StorageError,
};
use store_facade_model::FacadeError;

use crate::execution::Interruption;

/// Translates store errors into technology-neutral [`FacadeError`]s.
///
/// Storage errors become one fixed message, so no product, path or engine
/// detail reaches a caller.
pub(crate) struct ErrorTranslation;

impl ErrorTranslation {
    const MESSAGE: &'static str = "the store's storage failed";

    pub(crate) fn from_storage(_error: StorageError) -> FacadeError {
        FacadeError::backend(Self::MESSAGE)
    }

    pub(crate) fn from_syntax(error: &SparqlSyntaxError) -> FacadeError {
        FacadeError::invalid_query(error.to_string())
    }

    pub(crate) fn from_load(error: LoaderError) -> FacadeError {
        match error {
            LoaderError::Parsing(parse) => FacadeError::backend(format!("invalid data: {parse}")),
            LoaderError::Storage(storage) => Self::from_storage(storage),
            other @ LoaderError::InvalidBaseIri { .. } => FacadeError::backend(other.to_string()),
        }
    }

    pub(crate) fn from_query(
        error: QueryEvaluationError,
        interruption: &Interruption,
    ) -> FacadeError {
        match error {
            QueryEvaluationError::Cancelled => interruption.error(),
            QueryEvaluationError::Dataset(_) => FacadeError::backend(Self::MESSAGE),
            QueryEvaluationError::UnsupportedService(_)
            | QueryEvaluationError::UnsupportedCustomFunction(_)
            | QueryEvaluationError::UnsupportedCustomFunctionArity { .. } => {
                FacadeError::invalid_query(error.to_string())
            }
            other => FacadeError::backend(other.to_string()),
        }
    }

    pub(crate) fn from_update(
        error: UpdateEvaluationError,
        interruption: &Interruption,
    ) -> FacadeError {
        match error {
            UpdateEvaluationError::Storage(storage) => Self::from_storage(storage),
            UpdateEvaluationError::UnsupportedService(_) => {
                FacadeError::invalid_query(error.to_string())
            }
            other if interruption.is_interrupted() => {
                drop(other);
                interruption.error()
            }
            other => FacadeError::backend(other.to_string()),
        }
    }
}
