use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const MAX_BLANK: u32 = 3;
const MAX_CHARS: usize = 100;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        print!("Needs one or more files as arguments! For example, mike_formatter example.java");
    } else {
        for i in 1..args.len() {
            println!("Formatting file: {}\n", &args[i]);
            format(String::from(&args[i]));
            println!("\n\nDone!\n")
        }
    }
}

fn format(file_name: String) {
    if file_name.len() < 6 || !file_name[file_name.len() - 4..file_name.len()].eq("java") {
        println!("Input a java file!");
        return;
    }

    let mut contents = fs::read_to_string(&file_name).expect("File not found!");

//     debug_print(&mut contents);

    let changed = wrap(&mut contents);

    if changed {
        println!("Changes have been made.");
    }

    if changed {
        fs::remove_file(&file_name).expect("Filed deletion failed.");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(file_name)
            .expect("Couldn't open file!");

        write!(file, "{contents}").expect("Couldn't write!");
    }
}

fn remove_blank_lines(contents: &mut String) -> bool {
    *contents = if is_dos(&contents.chars().collect()) {
        remove_dos(contents)
    } else {
        remove_nix(contents)
    };

    true
}

fn is_dos(chars: &Vec<char>) -> bool {
    for c in chars {
        if *c == '\r' {
            return true;
        }
    }

    false
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

fn wrap(contents: &mut String) -> bool {
    let chars: &mut Vec<char> = &mut contents.chars().collect();

    let ret = wrap_around(chars);

    *contents = char_to_str(chars);

    ret
}

fn wrap_around(contents: &mut Vec<char>) -> bool {
    let special_chars: HashSet<char> = HashSet::from(['.', ',', '\\',
                                                      '&', '|', ':', '(', ')', '+', '=']);
    let mut changed = false;
    let mut lines = line_decomp(contents);
    for line in 0..lines.len() {
        if lines[line].len() <= MAX_CHARS {
            continue;
        }
        changed = true;
        println!("Modifying line number: {line}");
        let mut i = lines[line].len() - 1;
        let mut in_string = false;
        let mut new_line: LinkedList<char> = LinkedList::new();
        while i > 0 {
            if i == MAX_CHARS {
               if in_string {
                   new_line.push_front(lines[line][MAX_CHARS]);
                   new_line.push_front(lines[line][MAX_CHARS - 1]);
                   lines[line][MAX_CHARS] = '+';
                   lines[line][MAX_CHARS - 1] = '\"';
                   let l = lines[line].len();
                   lines[line].drain(MAX_CHARS + 1..l);
                   if *new_line.front().unwrap() != '\"' {
                       new_line.push_front('\"');
                   }
                   break;
               } 
            }
            if i <= 100 && special_chars.contains(&lines[line][i]) {
                new_line.push_front(lines[line][i]);
                let l = lines[line].len();
                lines[line].drain(i..l);
                break;
            } 

            if  lines[line][i] == '\"' {
                in_string = if in_string { false } else { true };
            }
            new_line.push_front(lines[line][i]);
            i -= 1;
        }

        indent(&mut new_line, indent_level(&lines[line]));
        lines.insert(line + 1, ll_to_vec(new_line));
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

fn collect_lines(lines: Vec<Vec<char>>, dos: bool) -> Vec<char> {
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

fn indent_level(line: &Vec<char>) -> u8{
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

fn line_decomp(contents: &Vec<char>) -> Vec<Vec<char>> {
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

fn _print_file(chars: &Vec<char>) {
    for c in chars {
        if *c == '\n' {
            print!("\\n");
        } else if *c == '\r' {
            print!("\\r");
        } else {
            print!("{c}");
        }
    }
}

fn char_to_str(chars: &Vec<char>) -> String {
    chars.iter().collect()
}
