use std::collections::HashSet;
enum WritingMode {
    None,
    Comment,
    Name,
    Int,
}

pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut mode: WritingMode = WritingMode::None;
    let mut buffer: String = String::new();
    for c in text.chars() {
        match mode {
            WritingMode::Comment => {
                if c != '\n' {
                    continue;
                } else {
                    mode = WritingMode::None;
                }
            }
            WritingMode::Name => {
                if c.is_alphanumeric() || c == '_' {
                    buffer.push(c);
                    continue;
                } else {
                    tokens.push(buffer.clone());
                    buffer.clear();
                    mode = WritingMode::None;
                }
            }
            WritingMode::Int => {
                if c == '_' {
                    continue;
                } else if c.is_numeric() {
                    buffer.push(c);
                    continue;
                } else {
                    tokens.push(buffer.clone());
                    buffer.clear();
                    mode = WritingMode::None;
                }
            }
            WritingMode::None => (),
        }
        match c {
            '_' | 'a'..='z' | 'A'..='Z' => {
                buffer.clear();
                buffer.push(c);
                mode = WritingMode::Name;
            }
            '#' => {
                mode = WritingMode::Comment;
            }
            _ if c.is_numeric() => {
                buffer.clear();
                buffer.push(c);
                mode = WritingMode::Int;
            }
            _ if c.is_ascii_punctuation() => {
                tokens.push(format!("{}", c));
            }
            _ => (),
        }
    }
    combine_tokens(tokens)
}

fn combine_tokens(tokens: Vec<String>) -> Vec<String> {
    let two_char_pairs: HashSet<&'static str> = [
        "==", "!=", "=!", "=<", "=>", "<=", "<!", ">=", ">!", "!<", "!>", "!&", "!|",
        "!^", // order shall not matter; ! inverts
    ]
    .iter()
    .copied()
    .collect();

    let mut out = Vec::with_capacity(tokens.len());
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "-"
            && i + 1 < tokens.len()
            && tokens[i + 1].chars().all(|c| c.is_ascii_digit())
        {
            out.push(format!("-{}", tokens[i + 1]));
            i += 2;
            continue;
        }

        if i + 1 < tokens.len() {
            let pair = format!("{}{}", tokens[i], tokens[i + 1]);
            if two_char_pairs.contains(pair.as_str()) {
                out.push(pair);
                i += 2;
                continue;
            }
        }

        out.push(tokens[i].clone());
        i += 1;
    }

    out
}
