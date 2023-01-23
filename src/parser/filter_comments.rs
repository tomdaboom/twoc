pub fn filter_comments(input : String) -> String {
    let lines = input.split("\n");

    let out_lines = lines.filter(
        |line| {
            for char in line.chars() {
                if char == '/' { return false; }
                else if !char.is_whitespace() { return true; }
            }

            true
    }
        
    );

    let mut out_string = "".to_owned();
    for line in out_lines {
        out_string.push_str(line);
    }

    out_string
}