use crate::{ConstructQueryPatterns, SparqlVariableGenerator};

/// A trait for types that can be constructed from SPARQL patterns.
pub trait Constructible {
    /// Builds the SPARQL patterns needed to construct this type.
    fn insert_patterns(
        construct_query_patterns: &mut ConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
    );
}
