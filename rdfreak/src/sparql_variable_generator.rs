/// Encodes a number in base 26 using letters 'a' to 'z'.
fn encode_base_26(mut n: usize) -> String {
    let mut result = Vec::new();

    while n > 0 {
        let rem = (n - 1) % 26;

        result.push((b'a' + rem as u8) as char);

        n = (n - 1) / 26;
    }

    result.iter().rev().collect()
}

#[derive(Debug, Clone, Default)]
pub struct SparqlVariableGenerator {
    current: usize,
}

impl SparqlVariableGenerator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Iterator for SparqlVariableGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        let next_base_26 = encode_base_26(self.current);
        let variable_name = format!("?{}", next_base_26);

        Some(variable_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_generator() {
        let mut generator = SparqlVariableGenerator::new();

        assert_eq!(generator.next(), Some("?a".to_string()));
        assert_eq!(generator.next(), Some("?b".to_string()));
        assert_eq!(generator.next(), Some("?c".to_string()));

        // jump to ?z
        assert_eq!(generator.nth(22), Some("?z".to_string()));

        // continue
        assert_eq!(generator.next(), Some("?aa".to_string()));
        assert_eq!(generator.next(), Some("?ab".to_string()));
        assert_eq!(generator.next(), Some("?ac".to_string()));

        // jump to ?az
        assert_eq!(generator.nth(22), Some("?az".to_string()));

        // continue
        assert_eq!(generator.next(), Some("?ba".to_string()));
        assert_eq!(generator.next(), Some("?bb".to_string()));
        assert_eq!(generator.next(), Some("?bc".to_string()));
    }
}
