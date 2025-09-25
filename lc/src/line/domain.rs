use super::types::Line;
use util::{file, split};

pub struct Domain {
    content: Vec<Line>,
    indent: Vec<usize>
}

impl Domain {
    pub fn load(path: &str) -> Vec<Vec<String>> {
        let data: Vec<String> = file::read_file(path);

        data
            .iter()
            .map(|line| split::split_line(line))
            .collect()
    }
}
