use oxrdf::NamedNode;

use crate::{FromRdf, FromResourceResult, Resource};

/// Deserializes all resources of type `T` from the given RDF graph.
pub fn deserialize_all<R: Resource + FromRdf>(
    graph: &oxrdf::Graph,
) -> impl Iterator<Item = FromResourceResult<R>> {
    let resource_rdf_type = R::get_rdf_type();

    let rdf_type_predicate =
        NamedNode::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");

    let subjects = graph
        .subjects_for_predicate_object(&rdf_type_predicate, &resource_rdf_type)
        .map(|s| s.into_owned())
        .collect::<Vec<_>>();

    subjects
        .into_iter()
        .map(move |subject| R::from_rdf(graph, &subject))
}
