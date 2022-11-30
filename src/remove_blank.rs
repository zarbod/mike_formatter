use crate::MAX_BLANK;
use crate::misc::is_dos;

pub fn remove_blank_lines(chars: &mut Vec<char>) -> bool {
    let old_len = chars.len();

    if is_dos(chars) {
        remove_dos(chars);
    } else {
        remove_nix(chars);
    }

    old_len != chars.len()
}

fn remove_nix(chars: &mut Vec<char>) {
    let mut i = 0;
    let mut count = 0;

    while i < chars.len() {
        if chars[i] == '\n' {
            count += 1;
        } else {
            count = 0;
        }

        if count >= MAX_BLANK {
            chars.remove(i);
            i -= 1;
        }
        i += 1;
    }
}

fn remove_dos(chars: &mut Vec<char>) {
    let mut i = 0;
    let mut count = 0;
    let mut incr;

    while i < chars.len() - 1 {
        incr = 1;

        if chars[i] == '\r' && chars[i + 1] == '\n' {
            count += 1;
            incr = 2;
        } else {
            count = 0;
        }

        if count >= MAX_BLANK {
            chars.drain(i..i+2);
            incr = 0;
        }
        i += incr;
    }
}
