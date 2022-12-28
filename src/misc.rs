pub fn is_dos(chars: &Vec<char>) -> bool {
    for c in chars {
        if *c == '\r' {
            return true;
        }
    }

    false
}

pub fn line_decomp(contents: &Vec<char>) -> Vec<Vec<char>> {
    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut line_index = 0;

    let incr = if is_dos(contents) {
        1
    } else {
        0
    };

    lines.push(Vec::new());
    let mut i = 0;
    while i < contents.len() {
        if contents[i] == '\n' || contents[i] == '\r' {
            lines.push(Vec::new());
            line_index += 1;
            i += incr;
        } else {
            lines[line_index].push(contents[i]);
        }
        i += 1;
    }

    lines
}

pub fn collect_lines(lines: Vec<Vec<char>>, dos: bool) -> Vec<char> {
    let mut contents: Vec<char> = Vec::new();
    for line in lines {
        for c in line {
            contents.push(c);
        }
        if dos { contents.push('\r'); }
        contents.push('\n');
    }
    contents
}
