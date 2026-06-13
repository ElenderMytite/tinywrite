use std::collections::HashSet;

#[derive(Debug)]
enum WritingMode {
    None,
    AfterNewline,
    Comment,
    Name,
    Int,
}

pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut mode: WritingMode = WritingMode::AfterNewline;
    let mut buffer: String = String::new();
    for c in text.chars() {
        match mode {
            WritingMode::AfterNewline => {
                if c == ';' {
                    mode = WritingMode::Comment;
                    continue;
                }
            }
            WritingMode::Comment => {
                if c != '\n' {
                    continue;
                } else {
                    mode = WritingMode::AfterNewline;
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
        let alpha = c.is_alphabetic() || c == '_';
        let num = c.is_numeric() || c == '-';
        let punct = c.is_ascii_punctuation();
        match c {
            _ if alpha => {
                buffer.clear();
                buffer.push(c);
                mode = WritingMode::Name;
            }
            _ if num => {
                buffer.clear();
                buffer.push(c);
                mode = WritingMode::Int;
            }
            ';' => {
                tokens.push(";".to_string());
                mode = WritingMode::Comment;
            }
            '\n' => {
                mode = WritingMode::AfterNewline;
            }
            _ if punct => {
                tokens.push(format!("{}", c));
            }
            _ => {
                mode = WritingMode::None;
            }
        }
    }
    if !buffer.is_empty() {
        tokens.push(buffer);
    }
    combine_tokens(tokens)
}

fn combine_tokens(tokens: Vec<String>) -> Vec<String> {
    let two_char_pairs: HashSet<&'static str> = [
        "==", "!=", "=!", "=<", "=>", "<=", ">=", "<!", ">!", "!<", "!>", "!&", "!|", "!^", ",,",
        "..",
    ]
    .iter()
    .copied()
    .collect();

    let mut out = Vec::with_capacity(tokens.len());
    let mut i = 0;
    while i < tokens.len() {
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
