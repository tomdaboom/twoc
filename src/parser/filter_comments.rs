pub fn filter_comments(input : String) -> String {
    let lines = input.split("\n");

    let out_lines = lines.filter(
        |_| {
            // TODO: Fix
            return true;
        }
    );

    let mut out_string = "".to_owned();
    for line in out_lines {
        out_string.push_str(line);
    }

    out_string
}