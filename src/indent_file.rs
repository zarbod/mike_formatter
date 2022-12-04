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

fn check_level(level: u8, line: &mut Vec<char>) {
    let mut k = 0;
    for j in 0..level {
        for i in 0..4 {
            if line[k] != ' ' {
                line.insert(k, ' ');
            }
            k += 1;
        }
    }
}
