#[derive(Debug, Clone, Default)]
pub struct SparqlConstructQueryPatterns {
    pub patterns: String,
    pub where_patterns: String,
}

impl SparqlConstructQueryPatterns {
    pub fn new() -> Self {
        Self::default()
    }
}
