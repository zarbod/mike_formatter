mod remove_blank;
mod wrap;
use wrap::wrap;
use remove_blank::remove_blank_lines;
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
            println!("\nDone!\n")
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

    let changed = wrap(&mut contents) | remove_blank_lines(&mut contents);

    if changed {
        println!("Changes have been made.");
    } else {
        println!("No changes were made.");
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

pub fn is_dos(chars: &Vec<char>) -> bool {
    for c in chars {
        if *c == '\r' {
            return true;
        }
    }

    false
}
