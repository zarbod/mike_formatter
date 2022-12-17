use crate::misc::{collect_lines, line_decomp, is_dos};

pub fn space(chars: &mut Vec<char>) -> bool {
    let mut lines = line_decomp(chars);
    let len = chars.len();
    for i in 0.. {
        if i == lines.len() { break; }
        for j in 0.. {
            if j == lines[i].len() { break; }
            if lines[i][j] == ':' {
                if j < lines[i].len() - 1 && lines[i][j + 1] != ' '{ 
                    lines[i].insert(j + 1, ' '); 
                }
                if lines[i][j - 1] != ' ' { lines[i].insert(j - 1, ' '); }
            }
        }
    }
    *chars = collect_lines(lines, is_dos(chars));
    len != chars.len()
}
