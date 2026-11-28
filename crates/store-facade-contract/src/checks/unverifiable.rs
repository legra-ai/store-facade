use store_facade_model::vocabulary::CapabilityTerm;

/// The model has no request surface for `capability` yet, so declaring it
/// cannot be held to anything and fails the contract.
pub(crate) fn unverifiable(capability: CapabilityTerm) {
    panic!(
        "<{}> has no observable contract yet; a binding must not declare it",
        capability.capability()
    );
}
