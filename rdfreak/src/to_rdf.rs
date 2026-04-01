use oxrdf::Graph;

/// A trait for converting a type into RDF triples and adding them to a graph.
pub trait ToRdf {
    /// Converts the implementing type into RDF triples and adds them to the provided graph.
    fn to_rdf(&self, graph: &mut Graph);
}
