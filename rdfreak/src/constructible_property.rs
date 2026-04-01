use oxrdf::NamedNode;

use crate::{ConstructQueryPatterns, SparqlVariableGenerator, TriplePattern, TriplePatternNode};

/// A trait for properties that can be constructed from SPARQL patterns.
pub trait ConstructibleProperty {
    /// Builds the SPARQL patterns needed to construct this property.
    fn insert_patterns(
        construct_query_patterns: &mut ConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
        predicate: &NamedNode,
    );
}

// implementations

// note: this implementation will be basically the exact same for all literals
impl ConstructibleProperty for String {
    fn insert_patterns(
        construct_query_patterns: &mut ConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
        predicate: &NamedNode,
    ) {
        let object_variable = variable_generator.next().unwrap();

        let triple_pattern = TriplePattern::new(
            TriplePatternNode::Variable(subject_variable.to_owned()),
            TriplePatternNode::NamedNode(predicate.clone()),
            TriplePatternNode::Variable(object_variable),
        );

        construct_query_patterns.push_identical_triple_pattern(triple_pattern);
    }
}

impl<T: ConstructibleProperty> ConstructibleProperty for Option<T> {
    fn insert_patterns(
        construct_query_patterns: &mut ConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
        predicate: &NamedNode,
    ) {
        let mut inner_patterns = ConstructQueryPatterns::new();

        T::insert_patterns(
            &mut inner_patterns,
            variable_generator,
            subject_variable,
            predicate,
        );

        construct_query_patterns
            .template_patterns
            .extend(inner_patterns.template_patterns);

        construct_query_patterns
            .where_pattern
            .push_optional(inner_patterns.where_pattern);
    }
}

impl<T: ConstructibleProperty> ConstructibleProperty for Vec<T> {
    fn insert_patterns(
        construct_query_patterns: &mut ConstructQueryPatterns,
        variable_generator: &mut SparqlVariableGenerator,
        subject_variable: &str,
        predicate: &NamedNode,
    ) {
        let mut inner_patterns = ConstructQueryPatterns::new();

        T::insert_patterns(
            &mut inner_patterns,
            variable_generator,
            subject_variable,
            predicate,
        );

        construct_query_patterns
            .template_patterns
            .extend(inner_patterns.template_patterns);

        construct_query_patterns
            .where_pattern
            .push_optional(inner_patterns.where_pattern);
    }
}
