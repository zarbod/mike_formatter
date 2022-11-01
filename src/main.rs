use std::env;
use std::fs;
use std::io::Write;
use std::fs::OpenOptions;

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


fn remove_blank_lines(contents: &mut String) -> bool {
    let mut break_counter: u32 = 0;
    let mut pos: usize = 0;
    let mut blank_list = Vec::new();
    let mut removed = false;
    for c in contents.chars() {
        pos += 1;
        if c == '\n' {
            break_counter += 1;
        } else {
            break_counter = 0;
        }

        if break_counter >= MAX_BLANK {
            blank_list.push(pos);
        }
    }

    if  blank_list.len() > 0  {
        removed = true;
        let mut num_removed = 0;
        println!("{}", blank_list.len());
        for i in blank_list {
            num_removed += 1;
            contents.remove(i - num_removed);
        }
    }

    return removed;
}

fn wrap_around(contents: &mut String) -> bool {
    let mut line_chars: u32 = 0;
    let mut line_count: u32 = 1;
    let mut bad_lines: Vec<u32> = Vec::new();

    for c in contents.chars() {
        line_chars += 1;
        if line_chars == MAX_CHARS + 1 && c != '\n'{
            bad_lines.push(line_count);
        } else if c == '\n' {
            line_count += 1;
            line_chars = 0;
        }
    }

    if bad_lines.len() > 0 {
        print!("Character limit exceeded on lines:");
        for i in &bad_lines {
            print!(" {i}");
        }
        print!("!!!");
    }
    return bad_lines.len() > 0;
}

fn format(file_name: String) {
    if file_name.len() < 6 ||
        !file_name[file_name.len() - 4..file_name.len()].eq("java") {
        println!("Input a java file!");
        return;
    }

    let mut contents = fs::read_to_string(&file_name).expect("File not found!");

    let changed = remove_blank_lines(&mut contents) ||
        wrap_around(&mut contents);

    if  changed {
        fs::remove_file(&file_name)
            .expect("Filed deletion failed.");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(file_name)
            .expect("Couldn't open file!");

        write!(file, "{contents}").expect("Couldn't write!");
    }

}
