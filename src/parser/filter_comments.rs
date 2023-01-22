pub fn filter_comments(input : &str) -> &str {
    let lines = input.split("\n");

    let out_lines = lines.filter(|line| 
        (line.chars().nth(0) == Some('/')) && (line.chars().nth(1) == Some('/')) 
    );

    let mut out_string = "".to_owned();
    for line in out_lines {
        out_string.push_str(line);
    }

    &*out_string
}