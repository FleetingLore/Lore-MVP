pub fn read_file(file_path: &str) -> Vec<String> {
    std::fs::read_to_string(std::path::PathBuf::from(file_path))
        .expect("[1_Lore util API]")
        .lines()
        .map(|s| s.to_string())
        .collect()
}
