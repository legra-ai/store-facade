use rdf_structural_model::{
    Literal,
    LiteralFacets,
    Subject,
    Term,
    Triple,
    XSD_STRING,
};

use crate::vocabulary::{
    RDF_TYPE,
    RDFS_LABEL,
    SF_SLUG,
    SF_STORE,
};
use crate::{
    Capability,
    CapabilityRole,
    CapabilitySet,
    FacadeError,
    StoreDescription,
    StoreIri,
    StoreLabel,
    StoreLabels,
    StoreSlug,
};

/// Reads one store's description from a stream of triples.
///
/// Feed every triple to [`StoreDescriptionReader::accept`] in any order, then
/// call [`StoreDescriptionReader::finish`]. Triples about other subjects are
/// skipped, so a reader can run over a larger graph in one pass. Memory stays
/// bounded by the description limits, never by the size of the input.
///
/// A triple about the store must be one the writer produces: unknown
/// predicates, extra types and non-IRI capabilities are rejected rather than
/// silently dropped.
#[derive(Debug, Clone)]
pub struct StoreDescriptionReader {
    iri: StoreIri,
    typed: bool,
    slug: Option<StoreSlug>,
    labels: StoreLabels,
    capabilities: CapabilitySet,
}

impl StoreDescriptionReader {
    /// A reader for the description of `iri`.
    #[must_use]
    pub const fn new(iri: StoreIri) -> Self {
        Self {
            iri,
            typed: false,
            slug: None,
            labels: StoreLabels::new(),
            capabilities: CapabilitySet::new(),
        }
    }

    /// Takes one triple; returns whether it was about the store.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidDescriptionRdf`] for a statement that cannot
    /// belong to a description, or a label, slug or limit error.
    pub fn accept(&mut self, triple: &Triple) -> Result<bool, FacadeError> {
        match triple.subject() {
            Subject::Iri(subject) if subject == self.iri.as_iri() => {}
            _ => return Ok(false),
        }
        let predicate = triple.predicate().as_str();
        let object = triple.object();
        match predicate {
            RDF_TYPE => self.accept_type(object)?,
            SF_SLUG => self.accept_slug(object)?,
            RDFS_LABEL => self
                .labels
                .insert(StoreLabel::try_from_literal(Self::literal(object)?)?)?,
            _ => self.accept_capability(predicate, object)?,
        }
        Ok(true)
    }

    /// Takes every triple of `triples`, stopping at the first error.
    ///
    /// # Errors
    ///
    /// As [`StoreDescriptionReader::accept`].
    pub fn accept_all<'triple>(
        &mut self,
        triples: impl IntoIterator<Item = &'triple Triple>,
    ) -> Result<(), FacadeError> {
        for triple in triples {
            self.accept(triple)?;
        }
        Ok(())
    }

    /// Completes the description.
    ///
    /// # Errors
    ///
    /// [`FacadeError::InvalidDescriptionRdf`] without `rdf:type sf:Store`,
    /// [`FacadeError::MissingSlug`] or [`FacadeError::MissingLabel`].
    pub fn finish(self) -> Result<StoreDescription, FacadeError> {
        if !self.typed {
            return Err(FacadeError::InvalidDescriptionRdf {
                reason: "a store description states rdf:type sf:Store",
            });
        }
        let slug = self.slug.ok_or(FacadeError::MissingSlug)?;
        StoreDescription::try_new(self.iri, slug, self.labels, self.capabilities)
    }

    fn accept_type(&mut self, object: &Term) -> Result<(), FacadeError> {
        match object {
            Term::Iri(class) if class.as_str() == SF_STORE => {
                self.typed = true;
                Ok(())
            }
            _ => Err(FacadeError::InvalidDescriptionRdf {
                reason: "the only rdf:type of a store is sf:Store",
            }),
        }
    }

    fn accept_slug(&mut self, object: &Term) -> Result<(), FacadeError> {
        let literal = Self::literal(object)?;
        if literal.metadata().datatype().as_str() != XSD_STRING
            || *literal.metadata().facets() != LiteralFacets::None
        {
            return Err(FacadeError::InvalidDescriptionRdf {
                reason: "a slug is an xsd:string",
            });
        }
        let slug = StoreSlug::try_new(literal.lexical())?;
        match &self.slug {
            Some(existing) if *existing != slug => Err(FacadeError::InvalidDescriptionRdf {
                reason: "a store has exactly one slug",
            }),
            _ => {
                self.slug = Some(slug);
                Ok(())
            }
        }
    }

    fn accept_capability(&mut self, predicate: &str, object: &Term) -> Result<(), FacadeError> {
        let Some(role) = CapabilityRole::from_predicate(predicate) else {
            return Err(FacadeError::InvalidDescriptionRdf {
                reason: "a store description uses only rdf:type, sf:slug, rdfs:label, \
                         sd:supportedLanguage, sd:feature and sd:entailmentRegime",
            });
        };
        let Term::Iri(iri) = object else {
            return Err(FacadeError::InvalidDescriptionRdf {
                reason: "a capability is an IRI",
            });
        };
        self.capabilities
            .insert(Capability::new(role, iri.clone()))?;
        Ok(())
    }

    fn literal(object: &Term) -> Result<&Literal, FacadeError> {
        match object {
            Term::Literal(literal) => Ok(literal),
            _ => Err(FacadeError::InvalidDescriptionRdf {
                reason: "slugs and labels are literals",
            }),
        }
    }
}

impl StoreDescription {
    /// Reads the description of `iri` from `triples`.
    ///
    /// # Errors
    ///
    /// As [`StoreDescriptionReader::accept`] and
    /// [`StoreDescriptionReader::finish`].
    pub fn from_triples<'triple>(
        iri: StoreIri,
        triples: impl IntoIterator<Item = &'triple Triple>,
    ) -> Result<Self, FacadeError> {
        let mut reader = StoreDescriptionReader::new(iri);
        reader.accept_all(triples)?;
        reader.finish()
    }
}
