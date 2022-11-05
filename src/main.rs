use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const MAX_BLANK: u32 = 3;
const MAX_CHARS: u32 = 100;

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

    let changed = remove_blank_lines(&mut contents);

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
    *contents = remove_dos(contents);
    return true;
}

fn remove_nix(contents: &String) -> String {

    let mut chars = char_decomposition(contents);

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

    return char_to_str(&chars);
}

fn remove_dos(contents: &String) -> String {

    let mut chars = char_decomposition(contents);

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
            chars.remove(i);
            chars.remove(i);
            incr = 0;
        }
        i += incr;
    }

    return char_to_str(&chars);
}

/*
fn wrap_around(contents: &mut String) -> bool {
    let split_chars: HashSet<u8> = HashSet::from([
        '.' as u8, ',' as u8, '\"' as u8, '\\' as u8, '&' as u8, '|' as u8, ':' as u8, '?' as u8,
        '(' as u8, ')' as u8,
    ]);
    let mut changed = false;
    let mut lines: Vec<String> = split_by_lines(contents);

    for i in 0..lines.len() {
        if lines[i].len() as u32 > MAX_CHARS {
            changed = true;
            let mut new_line: String = String::new();
            let mut j = lines[i].len() - 1;

            while !split_chars.contains(&(lines[i].as_bytes()[j])) && j > 0 {
                new_line += &(lines[i].as_bytes()[j] as char).to_string();
                j -= 1;
            }

            println!("{}", &new_line);
            lines.insert(i + 1, new_line);
        }
    }

    return changed;
}
*/

fn print_file(chars: &Vec<char>) {
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

    let mut ret_str = String::new();
    for c in chars {
        ret_str += &c.to_string();
    }

    return ret_str;
}

fn char_decomposition(contents: &String) -> Vec<char> {

    let mut chars: Vec<char> = Vec::new();

    for c in contents.chars() {
        chars.push(c);
    }

    return chars;
}
