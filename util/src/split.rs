pub fn split_line(line: &str) -> Vec<String> {
    line
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}
