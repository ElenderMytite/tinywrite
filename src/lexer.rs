const SPLITTERS: [char; 10] = ['$', ':', ';', '\'', '(', '{', '[', ']', '}', ')'];
const FINALS: [&'static str; 19] = [
    "?|", "!|", "++", "**", "||", "&&", "^^", "...", "#.", "@.", "|\\", "/|", "~>", "->", "<-",
    "</", "<_", "_>", "\\>",
];
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum WritingMode {
    Fresh,
    Comment,
    Backslash,
    String,
    Name,
    Special,
    Int,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Name(String),
    String(String),
    Int(isize),
    // parentheses
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClosed,
    // specifiers
    Constant,
    Type,
    Call,
    EndOfStatement,
    // unary operators
    Negation,
    // binary operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,
    And,
    Or,
    Xor,
    Nand,
    Nor,
    Equality,
    Unequality,
    StrictLess,
    LooseLess,
    StrictMore,
    LooseMore,
    Group,
    Composition,
    // collectors (opreators with unknown amount of elements)
    Sum,
    Product,
    Union,
    Intersection,
    SymmetricDiff,
    Glue,
    Slice,
}
impl Token {
    fn from_symbols(symbols: &str) -> Result<Self, SyntaxError> {
        match symbols {
            "$" => Ok(Token::Constant),
            ":" => Ok(Token::Call),
            ";" => Ok(Token::EndOfStatement),
            "'" => Ok(Token::Type),
            "(" => Ok(Token::RoundOpen),
            ")" => Ok(Token::RoundClose),
            "[" => Ok(Token::SquareOpen),
            "]" => Ok(Token::SquareClose),
            "{" => Ok(Token::CurlyOpen),
            "}" => Ok(Token::CurlyClosed),
            "+" => Ok(Token::Plus),
            "-" => Ok(Token::Minus),
            "*" => Ok(Token::Asterisk),
            "/" => Ok(Token::Slash),
            "%" => Ok(Token::Modulo),
            "=" => Ok(Token::Assign),
            "&" => Ok(Token::And),
            "!&" => Ok(Token::Nand),
            "!|" => Ok(Token::Nor),
            "|" => Ok(Token::Or),
            "^" => Ok(Token::Xor),
            "==" => Ok(Token::Equality),
            "!=" => Ok(Token::Unequality),
            ">" => Ok(Token::StrictMore),
            "<" => Ok(Token::StrictLess),
            ">=" => Ok(Token::LooseMore),
            "<=" => Ok(Token::LooseLess),
            "," => Ok(Token::Group),
            "." => Ok(Token::Composition),
            _ => Err(SyntaxError::UnkownSymbolicLiteral(symbols.to_string())),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxError {
    NonAscii(char),
    UnexpectedCharacterAfterBackslash(char),
    UnexpectedCharacter(char),
    UnkownSymbolicLiteral(String),
    ParseIntError(ParseIntError),
    EndlessString,
}
impl From<ParseIntError> for SyntaxError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
pub fn tokenize(text: &Vec<u8>) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens = Vec::new();
    let mut word = String::new();
    let mut mode = WritingMode::Fresh;
    let mut index = 0;
    while index < text.len() {
        let character = text[index] as char;
        if !character.is_ascii() {
            return Err(SyntaxError::NonAscii(character));
        }
        match (mode, character) {
            (WritingMode::Fresh, sep) if SPLITTERS.contains(&sep) => {
                tokens.push(Token::from_symbols(&sep.to_string())?);
            }
            (WritingMode::Fresh, '\"') => mode = WritingMode::String,
            (WritingMode::Comment, '\n') => mode = WritingMode::Fresh,
            (WritingMode::Comment, _) => (),
            (WritingMode::Fresh, '\n' | '\t' | ' ') => (),
            (WritingMode::Fresh | WritingMode::Name, '_' | 'a'..='z' | 'A'..='Z') => {
                word.push(character);
                mode = WritingMode::Name;
            }
            (WritingMode::Name, '0'..='9') => word.push(character),
            (WritingMode::Fresh | WritingMode::Int, '0'..='9') => {
                word.push(character);
                mode = WritingMode::Int;
            }
            (WritingMode::Special, ';') => {
                tokens.push(Token::from_symbols(word.as_str())?);
                tokens.push(Token::EndOfStatement); // because not in start mode, word isn't empty
                word = String::new();
                mode = WritingMode::Comment;
                continue;
            }
            (WritingMode::Special, sep) if SPLITTERS.contains(&sep) => {
                tokens.push(Token::from_symbols(word.as_str())?); // because not in start mode, word isn't empty
                word.clear();
                mode = WritingMode::Fresh;
                continue;
            }
            (WritingMode::Special | WritingMode::Fresh, symbol)
                if symbol.is_ascii_punctuation() =>
            {
                word.push(character);
                mode = WritingMode::Special;
                if FINALS.contains(&word.as_str()) {
                    tokens.push(Token::from_symbols(word.as_str())?); // because not in start mode, word isn't empty
                    word.clear();
                    mode = WritingMode::Fresh;
                }
            }
            (WritingMode::Special, _) => {
                tokens.push(Token::from_symbols(word.as_str())?); // because not in start mode, word isn't empty
                word = String::new();
                mode = WritingMode::Fresh;
                continue;
            }
            (WritingMode::String, '\\') => mode = WritingMode::Backslash,
            (WritingMode::String, '\"') => {
                tokens.push(Token::String(word));
                word = String::new();
                mode = WritingMode::Fresh;
            }
            (WritingMode::String, _) => {
                word.push(character);
            }
            (WritingMode::Backslash, 'n') => {
                word.push('\n');
                mode = WritingMode::String;
            }
            (WritingMode::Backslash, 't') => {
                word.push('\t');
                mode = WritingMode::String;
            }
            (WritingMode::Backslash, '\"') => {
                word.push('\"');
                mode = WritingMode::String;
            }
            (WritingMode::Backslash, '\'') => {
                word.push('\'');
                mode = WritingMode::String;
            }
            (WritingMode::Backslash, _) => {
                return Err(SyntaxError::UnexpectedCharacterAfterBackslash(character));
            }

            (WritingMode::Int, ';') => {
                tokens.push(Token::Int(word.parse()?));
                tokens.push(Token::EndOfStatement);
                word = String::new();
                mode = WritingMode::Comment;
            }
            (WritingMode::Name, ';') => {
                tokens.push(Token::Name(word));
                tokens.push(Token::EndOfStatement);
                word = String::new();
                mode = WritingMode::Comment;
            }
            (_, ';') => {
                unreachable!();
            }
            (WritingMode::Int, _) => {
                tokens.push(Token::Int(word.parse()?));
                word = String::new();
                mode = WritingMode::Fresh;
                continue;
            }
            (WritingMode::Name, _) => {
                tokens.push(Token::Name(word));
                word = String::new();
                mode = WritingMode::Fresh;
                continue;
            }
            (_, _) => return Err(SyntaxError::UnexpectedCharacter(character)),
        }
        index += 1;
    }
    if mode == WritingMode::String {
        return Err(SyntaxError::EndlessString);
    }
    Ok(tokens)
}
