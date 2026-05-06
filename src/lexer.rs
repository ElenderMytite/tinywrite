enum WritingMode {
    None,
    Name,
    Int,
}

pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut mode: WritingMode = WritingMode::None;
    let mut buffer: String = String::new(); 
    for c in text.chars() {
        match mode {
            WritingMode::Name => {
                if c.is_alphanumeric() || c == '_'{
                    buffer.push(c);
                    continue;
                }
                else {
                    tokens.push(buffer.clone());
                    buffer.clear();
                    mode = WritingMode::None;
                }
            }
            WritingMode::Int => {
                if c == '_' {
                    continue;
                }
                else if c.is_numeric() {
                    buffer.push(c);
                    continue;
                }
                else {
                    tokens.push(buffer.clone());
                    buffer.clear();
                    mode = WritingMode::None;
                }
            }
            WritingMode::None => ()
        }
        match c {
            '_' | 'a'..='z' | 'A'..='Z' => {
                buffer.clear();
                buffer.push(c);
                mode = WritingMode::Name;
            }
            _ if c.is_numeric() => {
                buffer.clear();
                buffer.push(c);
                mode = WritingMode::Int;
            }
            '+' | '-' | '*' | '/' | '|' | '(' | ')' | '[' | ']' | '{' | '}' | '=' | '<' | '>' => {
                tokens.push(format!("{}", c));
            }
            _ => ()
        }
    }
    tokens
}
