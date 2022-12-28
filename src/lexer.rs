use std::collections::{HashSet, HashMap};
use crate::misc::TOKEN;

pub fn lex(contents: &Vec<char>) -> Vec<(String, &TOKEN)> {
    let mut ret_vec: Vec<(String, &TOKEN)> = Vec::new();
    let mut i = 0;
    let terminals = gen_terminals();
    let keywords = gen_keywords();

    let mut curr_word: String = String::from("");

    while i < contents.len() {

        if terminals.contains_key(&contents[i]) { // Case where we have reached a terminal symbol
            let token = terminals.get(&contents[i]).unwrap();
            if !curr_word.eq("") {
                if keywords.contains(&curr_word) {
                    ret_vec.push((curr_word, &TOKEN::KEYWORD));
                } else {
                    ret_vec.push((curr_word, &TOKEN::ID));
                }
            }
            if **token != TOKEN::BLANK { // check if token has lexical value
                ret_vec.push((contents[i].to_string(), token));
            }
            curr_word = String::from(""); // reset word
        }

        curr_word.push(contents[i]);
        i += 1;
    }

    ret_vec
}

fn gen_keywords() -> HashSet<String> {
    HashSet::from([
        String::from("abstract"), String::from("assert"),       String::from("boolean"),  String::from("break"),
        String::from("byte"),     String::from("case"),         String::from("catch"),    String::from("char"),
        String::from("class"),    String::from("const"),        String::from("continue"), String::from("default"),
        String::from("do"),       String::from("double"),       String::from("else"),     String::from("enum"),
        String::from("extends"),  String::from("final"),        String::from("finally"),  String::from("float"),
        String::from("for"),      String::from("goto"),         String::from("if"),       String::from("implements"),
        String::from("import"),   String::from("instanceof"),   String::from("int"),      String::from("interface"),
        String::from("long"),     String::from("native"),       String::from("new"),      String::from("package"),
        String::from("private"),  String::from("protected"),    String::from("public"),   String::from("return"),
        String::from("short"),    String::from("static"),       String::from("strictfp"), String::from("super"),
        String::from("switch"),   String::from("synchronized"), String::from("this"),     String::from("throw"),
        String::from("throws"),   String::from("transient"),    String::from("try"),      String::from("void"),
        String::from("volatile"), String::from("while")
    ])
}

fn gen_terminals() -> HashMap<char, &'static TOKEN> {
    HashMap::from([
        ('+', &TOKEN::PLUS),
        ('-', &TOKEN::MINUS),
        ('*', &TOKEN::TIMES),
        (';', &TOKEN::SEMI),
        (',', &TOKEN::COMMA),
        ('{', &TOKEN::BRACEL),
        ('}', &TOKEN::BRACER),
        ('(', &TOKEN::PARENL),
        (')', &TOKEN::PARENR),
        ('[', &TOKEN::BRACKL),
        (']', &TOKEN::BRACKR),
        ('.', &TOKEN::DOT),
        (' ', &TOKEN::BLANK),
        ('\n', &TOKEN::BLANK),
        ('\r', &TOKEN::BLANK)
    ])
}
