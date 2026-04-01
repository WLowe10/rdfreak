use std::fmt::{self, Display};

use oxrdf::NamedNode;

#[derive(Debug, Clone)]
pub enum TriplePatternNode {
    Variable(String),
    NamedNode(NamedNode),
}

impl Display for TriplePatternNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TriplePatternNode::Variable(var) => write!(f, "?{}", var),
            TriplePatternNode::NamedNode(node) => write!(f, "{}", node),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TriplePattern {
    subject: TriplePatternNode,
    predicate: TriplePatternNode,
    object: TriplePatternNode,
}

impl TriplePattern {
    pub fn new(
        subject: TriplePatternNode,
        predicate: TriplePatternNode,
        object: TriplePatternNode,
    ) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }

    pub fn subject(&self) -> &TriplePatternNode {
        &self.subject
    }

    pub fn predicate(&self) -> &TriplePatternNode {
        &self.predicate
    }

    pub fn object(&self) -> &TriplePatternNode {
        &self.object
    }
}

impl Display for TriplePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} .", self.subject, self.predicate, self.object)
    }
}

#[derive(Debug, Clone)]
pub enum GraphPattern {
    Triples(Vec<TriplePattern>),
    Optional(Vec<GraphPattern>),
}

impl GraphPattern {
    pub fn push_triple_pattern(&mut self, triple_pattern: TriplePattern) {
        match self {
            GraphPattern::Triples(triples) => triples.push(triple_pattern),
            GraphPattern::Optional(graph_patterns) => {
                // check for an existing GraphPattern::Triples and add to it if it exists, otherwise create a new one

                let maybe_existing_triple_patterns =
                    graph_patterns.iter_mut().find_map(|graph_pattern| {
                        if let GraphPattern::Triples(triple_patterns) = graph_pattern {
                            Some(triple_patterns)
                        } else {
                            None
                        }
                    });

                if let Some(triple_patterns) = maybe_existing_triple_patterns {
                    triple_patterns.push(triple_pattern);
                } else {
                    graph_patterns.push(GraphPattern::Triples(vec![triple_pattern]));
                }
            }
        }
    }

    pub fn push_optional(&mut self, pattern: GraphPattern) {
        match self {
            GraphPattern::Triples(_) => {
                *self = GraphPattern::Optional(vec![std::mem::take(self), pattern]);
            }
            GraphPattern::Optional(patterns) => {
                patterns.push(pattern);
            }
        }
    }
}

impl Default for GraphPattern {
    fn default() -> Self {
        GraphPattern::Triples(vec![])
    }
}

impl Display for GraphPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphPattern::Triples(triple_pattern) => {
                for triple in triple_pattern {
                    write!(f, "{} ", triple)?;
                }

                Ok(())
            }
            GraphPattern::Optional(graph_patterns) => {
                write!(f, "OPTIONAL {{ ")?;

                for graph_pattern in graph_patterns {
                    write!(f, "{} ", graph_pattern)?;
                }

                write!(f, "}} ")
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConstructQueryPatterns {
    pub template_patterns: Vec<TriplePattern>,
    pub where_pattern: GraphPattern,
}

impl ConstructQueryPatterns {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_identical_triple_pattern(&mut self, triple_pattern: TriplePattern) {
        self.template_patterns.push(triple_pattern.clone());
        self.where_pattern.push_triple_pattern(triple_pattern);
    }

    // pub fn merge(&mut self, other: ConstructQueryPatterns) {
    //     self.template_patterns.extend(other.template_patterns);
    //     self.where_patterns.extend(other.where_patterns);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triple_pattern_display() {
        let triple_pattern = TriplePattern {
            subject: TriplePatternNode::Variable("s".to_string()),
            predicate: TriplePatternNode::NamedNode(
                NamedNode::new("http://example.com/p").unwrap(),
            ),
            object: TriplePatternNode::Variable("o".to_string()),
        };

        assert_eq!(triple_pattern.to_string(), "?s <http://example.com/p> ?o .");
    }
}
