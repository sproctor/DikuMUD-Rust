use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

use diku::interpreter::fill_word;

pub fn build_help_index(file: &File) -> HashMap<String, u64> {
    let table = HashMap::new();

    loop {
        let pos = file.seek(SeekFrom::Current(0)).expect("seek");
        let mut buf = String::new();
        if !fgets(&mut buf, 80, file) {
            break;
        }
        let mut scan: &str = &buf[0..];
        loop {
            let result = one_word(&scan);
            scan = result.0;
            let word = result.1;

            if scan.is_empty() {
                break;
            }
            
            table.insert(word, pos);
        }

        loop {
            buf.clear();
            if !fgets(&mut buf, 80, file) || buf.chars().nth(0) == Some('#') {
                break;
            }
        }
        if buf.chars().nth(1) == Some('#') {
            break;
        }
    }
    table
}

fn one_word<'a>(argument: &'a str) -> (&'a str, String) {
    let mut first_arg = String::new();
    let mut begin = 0;

    loop {
        first_arg.clear();
        begin += starting_whitespace(&argument[begin..]);

        if begin >= argument.len() {
            break;
        }
        if argument.chars().nth(begin).unwrap() == '"' {
            begin += 1;
            for c in argument[begin..].chars() {
                if c < ' ' { break; }
                begin += 1;
                if c == '"' { break; }
                first_arg.push_str(&c.to_lowercase().to_string());
            }
        } else {
            for c in argument[begin..].chars() {
                if c <= ' ' { break; }
                begin += 1;
                first_arg.push_str(&c.to_lowercase().to_string());
            }
        }
        if !fill_word(&first_arg) {
            break;
        }
    }
    (&argument[begin..], first_arg)
}

fn starting_whitespace(argument: &str) -> usize {
    let mut pos = 0;
    for c in argument.chars() {
        if !c.is_whitespace() { return pos; }
        pos += 1;
    }
    // string is all whitespace
    return pos;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_test() {
        assert_eq!(0, starting_whitespace("foobar"));
        assert_eq!(3, starting_whitespace("   foobar"));
    }

    #[test]
    fn one_word_test() {
        assert_eq!(("\r\n", String::from("help")), one_word("HELP\r\n"));
        assert_eq!(("", String::from("foobar")), one_word("foobar"));
        assert_eq!(("", String::from("hello world")), one_word("\"HELLO WORLD\""));
        assert_eq!(("foobar", String::from("hello world")), one_word("\"HELLO WORLD\"foobar"));
        assert_eq!((" foobar", String::from("hello world")), one_word("\"HELLO WORLD\" foobar"));
        assert_eq!(("  ", String::from("hello world")), one_word("   \"HELLO WORLD\"  "));
        assert_eq!((" bar", String::from("foo")), one_word("foo bar"));
        assert_eq!((" \"HELLO WORLD\"\r\n", String::from("test")), one_word("TEST \"HELLO WORLD\"\r\n"));
    }
}

// c fgets equivalent, except max is the number of characters to read, not characters + 1
fn fgets(dst: &mut String, max: usize, fp: &File) -> bool {
    let mut c: [u8; 1];
    let mut p: Vec<u8> = Vec::new();

    while max != 0 {
        if fp.read_exact(&mut c).is_err() {
            break;
        }
        p.push(c[0]);
        if c[0] == b'\n' {
            break;
        }
    }

    dst.push_str(&String::from_utf8(p).expect("Invalid UTF-8"));

    return p.len() == 0;
}