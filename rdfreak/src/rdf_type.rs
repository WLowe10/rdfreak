use oxrdf::{Graph, NamedNode, Term, Triple};

use crate::{
    DeserializeRdfObjectError, DeserializeRdfObjectResult, DeserializeRdfPropertyError,
    DeserializeRdfPropertyResult, RdfObject, RdfProperty,
};

#[derive(Debug, Clone)]
pub struct RdfType {
    value: NamedNode,
}

impl RdfType {
    pub fn new(value: NamedNode) -> Self {
        Self { value }
    }

    pub fn get_named_node(&self) -> &NamedNode {
        &self.value
    }
}

impl RdfObject for RdfType {
    fn to_term(&self) -> Term {
        Term::NamedNode(self.value.clone())
    }

    fn from_term(_graph: &Graph, term: &oxrdf::Term) -> DeserializeRdfObjectResult<Self> {
        let Term::NamedNode(named_node) = term else {
            return Err(DeserializeRdfObjectError::UnexpectedTermType(term.clone()));
        };

        Ok(Self::new(named_node.clone()))
    }
}

impl RdfProperty for RdfType {
    fn serialize_property(
        &self,
        graph: &mut Graph,
        subject: &oxrdf::NamedOrBlankNode,
        predicate: &NamedNode,
    ) {
        graph.insert(&Triple::new(
            subject.as_ref(),
            predicate.as_ref(),
            self.to_term(),
        ));
    }

    fn deserialize_property(
        graph: &Graph,
        subject: &oxrdf::NamedOrBlankNode,
        predicate: &NamedNode,
    ) -> DeserializeRdfPropertyResult<Self> {
        let maybe_object_term = graph.object_for_subject_predicate(subject, predicate);

        let Some(object_term) = maybe_object_term else {
            return Err(DeserializeRdfPropertyError::MissingObjectValue(
                predicate.clone(),
            ));
        };

        let rdf_type = Self::from_term(graph, &object_term.into())?;

        Ok(rdf_type)
    }
}
