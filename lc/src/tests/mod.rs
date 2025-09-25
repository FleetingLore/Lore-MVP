#[cfg(test)]
mod tests {
    #[test]
    fn test_domain_load() {
        // Import models.
        use crate::line::domain;

        // Construct data for test.
        let result = domain::Domain::load("./examples/example.txt");

        // Check data.
        for (i, line) in result.iter().enumerate() {
            println!("Line {i}: {:?}", line);
        }
    }
}
