use std::cmp::max;

const FILL: &'static [&'static str] = &[
    "in",
    "from",
    "with",
    "the",
    "on",
    "at",
    "to",
    ];

fn search_block(arg: &str, list: &[&str], exact: bool) -> Option<usize> {
    let word = arg.to_lowercase();

    if exact {
        for (i, &item) in list.iter().enumerate() {
            if word == item {
                return Some(i);
            }
        }
    } else {
        let length = max(word.len(), 1);
        for (i, &item) in list.iter().enumerate() {
            if word == item[0..length] {
                return Some(i);
            }
        }
    }
    
    None
}

pub fn fill_word(argument: &str) -> bool {
    search_block(argument, FILL, true).is_some()
}