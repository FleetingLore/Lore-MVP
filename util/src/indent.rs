use std::ops::Div;

pub fn indent_line(line: &str) -> usize {
        line
            .chars()
            .take_while(|c| c.is_whitespace())
            .count()
            .div(2)
}
