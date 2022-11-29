use crate::{MAX_BLANK, char_to_str, is_dos};

pub fn remove_blank_lines(contents: &mut String) -> bool {
    let old_len = contents.len();
    *contents = if is_dos(&contents.chars().collect()) {
        remove_dos(contents)
    } else {
        remove_nix(contents)
    };

    old_len != contents.len()
}

fn remove_nix(contents: &String) -> String {

    let chars: &mut Vec<char> = &mut contents.chars().collect();

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


    char_to_str(chars)
}

fn remove_dos(contents: &String) -> String {

    let chars: &mut Vec<char> = &mut contents.chars().collect();

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

    char_to_str(chars)
}
