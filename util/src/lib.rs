pub mod file;
pub mod split;
mod indent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        // Import models.
        use file::read_file;
        
        // Construct data for test.
        let data: Vec<String> = read_file("./examples/example.txt");
        
        // Check data.
        println!("[test_read_file]");
        println!("{}", data.join("\n"));
    }

    #[test]
    fn test_split_line() {
        // Import models.
        use split::split_line;

        // Construct data for test.
        let test_string = "a b c def g h i j k l    m      n";
        let split_test_string = split_line(test_string);

        // Check data.
        println!("[test_split_line]");
        for (i, tokens) in split_test_string.iter().enumerate() {
            println!("{i} |  {:?}", tokens);
        }
    }

    #[test]
    fn test_indent_line() {
        // Import models.
        use indent::indent_line;

        // Construct data for test.
        let test_cases = vec![
            "0",
            " 1_Lore util API",
            "  2",
            "   3",
            "    4",
            "     5",
            "   "
        ];

        // Check data.
        println!("[test_indent_line]");
        for (i, case) in test_cases.iter().enumerate() {
            let indent_count = indent_line(case);
            println!("{i} | [{}] -> {} indents", case, indent_count);
        }
    }
}

