use oxigraph::sparql::PreparedSparqlUpdate;
use oxigraph::store::Store;
use store_facade_model::FacadeError;

use super::Interruption;
use crate::error::ErrorTranslation;

/// Applies one update atomically on a blocking thread.
///
/// Every operation of the update runs in one transaction, committed only
/// when all succeed and no interruption arrived; otherwise the transaction
/// is dropped and the store is unchanged.
pub(crate) struct UpdateRun {
    store: Store,
    update: PreparedSparqlUpdate,
    interruption: Interruption,
}

impl UpdateRun {
    pub(crate) const fn new(
        store: Store,
        update: PreparedSparqlUpdate,
        interruption: Interruption,
    ) -> Self {
        Self {
            store,
            update,
            interruption,
        }
    }

    pub(crate) fn run(self) -> Result<(), FacadeError> {
        let _released = self.interruption.release_on_drop();
        let mut transaction = self
            .store
            .start_transaction()
            .map_err(ErrorTranslation::from_storage)?;
        self.update
            .on_transaction(&mut transaction)
            .execute()
            .map_err(|error| ErrorTranslation::from_update(error, &self.interruption))?;
        if self.interruption.is_interrupted() {
            return Err(self.interruption.error());
        }
        transaction.commit().map_err(ErrorTranslation::from_storage)
    }
}
