use crate::MAX_CHARS;
use crate::misc::{is_dos, line_decomp, collect_lines};
use std::collections::HashSet;
use std::collections::LinkedList;

pub fn wrap_around(contents: &mut Vec<char>) -> bool {
    // characters where breaking the line is allowed
    let special_chars: HashSet<char> = HashSet::from(['.', ',', '\\', '&', '|', ':', '(', ')', '+', '=']);
    let mut changed = false;
    let mut lines = line_decomp(contents);
    let mut line = 0;
    while line < lines.len() {
        if lines[line].len() > MAX_CHARS {
            changed = true;
            let mut i = lines[line].len() - 1;
            let mut in_string = false;
            let mut new_line: LinkedList<char> = LinkedList::new(); // O(1) push_front

            while i > 0 {
                if i == MAX_CHARS - 1{
                    if in_string {
                        new_line.push_front(lines[line][i]);
                        new_line.push_front(lines[line][i - 1]);
                        new_line.push_front(lines[line][i - 2]);
                        lines[line][i] = '+';
                        lines[line][i - 1] = ' ';
                        lines[line][i - 2] = '\"';
                        let length = lines[line].len();
                        lines[line].drain(MAX_CHARS..length);
                        if *new_line.front().unwrap() != '\"' {
                            new_line.push_front('\"');
                        }
                        break;
                    }
                }

                if i <= 100 && special_chars.contains(&lines[line][i]) {
                    let length = lines[line].len();
                    lines[line].drain((i + 1)..length);
                    break;
                }

                if  lines[line][i] == '\"' {
                    in_string = !in_string;
                }
                new_line.push_front(lines[line][i]);
                i -= 1;
            }

            indent(&mut new_line, indent_level(&lines[line]));
            lines.insert(line + 1, ll_to_vec(new_line));
        }
        line += 1;
    }

    if changed {
        *contents = collect_lines(lines, is_dos(contents));
    }

    changed
}

fn ll_to_vec(line: LinkedList<char>) -> Vec<char> {
    let mut ret: Vec<char> = Vec::new();
    for c in line {
        ret.push(c);
    }
    ret
}

fn indent_level(line: &Vec<char>) -> u8 {
    let mut count: u8 = 0;
    for c in line {
        if *c == ' ' {
            count += 1;
        } else { break; }
    }

    count / 4
}

fn indent(line: &mut LinkedList<char>, indent_level: u8) {
    for _i in 0..(indent_level + 1) {
        for _j in 0..4 {
            line.push_front(' ');
        }
    }
}
