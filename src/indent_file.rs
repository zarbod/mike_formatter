use crate::misc::{collect_lines, line_decomp, is_dos};

pub fn indent_file(chars: &mut Vec<char>) -> bool {
    let mut level = 0;
    let mut lines = line_decomp(chars);
    let mut level_inc = false;
    let mut changed = false;

    for i in 0.. {
        if i == lines.len() { break; }
        for j in 0.. {
            if j == lines[i].len() { break; }
            if lines[i][j] == '{' {
                level_inc = true;
            } else if lines[i][j] == '}' {
                level -= 1;
            }
        }

        if process_line(level, &mut lines[i]) {
            changed = true;
        }
        if level_inc {level += 1;}
        level_inc = false;
    }

    if changed { *chars = collect_lines(lines, is_dos(chars)); }

    changed
}

fn process_line(level: i32, line: &mut Vec<char>) -> bool {
    let mut val = false;
    let mut k = 0;
    if line.len() == 0 {return false;}
    for _j in 0..level {
        for _i in 0..4 {
            if line[k] != ' ' {
                line.insert(k, ' ');
                val = true;
            }
            k += 1;
        }
    }
    val
}
