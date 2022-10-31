use std::env;
use std::fs;
use std::io::Write;
use std::fs::OpenOptions;

const MAX_BLANK: u32 = 3;

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

fn format(file_name: String) {
    if file_name.len() < 6 ||
        !file_name[file_name.len() - 4..file_name.len()].eq("java") {
        println!("Input a java file!");
        return;
    }

    let mut contents = fs::read_to_string(&file_name).expect("File not found!");

    if remove_blank_lines(&mut contents) {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        print!("Needs one or more files as arguments! For example, mike_formatter example.java");
    } else {
        for i in 1..args.len() {
            format(String::from(&args[i]));
        }
    }
}
