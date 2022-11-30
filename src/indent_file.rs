use crate::misc::line_decomp;

fn indent_file(chars: &mut Vec<char>) {
    let mut level = 0;
    let mut lines = line_decomp(chars);

    for i in 0.. {
        if i == lines.len() { break; }
        for j in 0.. {
            if j == lines[i].len() { break; }
            if lines[i][j] == '{' {
                level += 1;
            } else if lines[i][j] == '}' {
                level -= 1;
            }
        }
    }
}
