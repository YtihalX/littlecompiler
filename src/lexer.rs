#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParanthesis,
    CloseParanthesis,
    Semicolon,
    Colon,
    Keyword(Keyword),
    Identifier(String),
    Literal(isize),
    Operator(Operator),
    Arrow,
    Unrecognizable,
    Comma,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Type(Type),
    Return,
    Fn,
    Let,
    If,
    Else,
    Match,
    While,
    For,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Isize,
    Usize,
    I64,
    U64,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Equal,
    LT,
    GT,
    LE,
    GE,
    Add,
    Sub,
    Assign,
    Mul,
    Div,
    Dot,
    LShift,
    RShift,
}


pub fn lex(code: &[u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let items = itemize(code);
    for item in items {
        tokens.push(match item {
            b"fn" => Token::Keyword(Keyword::Fn),
            b"return" => Token::Keyword(Keyword::Return),
            b"isize" => Token::Keyword(Keyword::Type(Type::Isize)),
            b"usize" => Token::Keyword(Keyword::Type(Type::Usize)),
            b"i64" => Token::Keyword(Keyword::Type(Type::I64)),
            b"u64" => Token::Keyword(Keyword::Type(Type::U64)),
            b"let" => Token::Keyword(Keyword::Let),
            b"if" => Token::Keyword(Keyword::If),
            b"else" => Token::Keyword(Keyword::Else),
            b"match" => Token::Keyword(Keyword::Match),
            b"while" => Token::Keyword(Keyword::While),
            b"for" => Token::Keyword(Keyword::For),
            b"->" => Token::Arrow,
            b"<" => Token::Operator(Operator::LT),
            b">" => Token::Operator(Operator::GT),
            b";" => Token::Semicolon,
            b":" => Token::Colon,
            b"(" => Token::OpenParanthesis,
            b")" => Token::CloseParanthesis,
            b"{" => Token::OpenBrace,
            b"}" => Token::CloseBrace,
            b"==" => Token::Operator(Operator::Equal),
            b"*" => Token::Operator(Operator::Mul),
            b"/" => Token::Operator(Operator::Div),
            b"+" => Token::Operator(Operator::Add),
            b"-" => Token::Operator(Operator::Sub),
            b"<=" => Token::Operator(Operator::LE),
            b">=" => Token::Operator(Operator::GE),
            b"=" => Token::Operator(Operator::Assign),
            b"." => Token::Operator(Operator::Dot),
            b"<<" => Token::Operator(Operator::LShift),
            b">>" => Token::Operator(Operator::RShift),
            b"," => Token::Comma,
            other => {
                if let Ok(literal) = String::from_utf8(other.to_vec()).unwrap().parse::<isize>() {
                    Token::Literal(literal)
                } else if punctuation_u8(other[0]) || assemblable_u8(other[0]) {
                    Token::Unrecognizable
                } else {
                    Token::Identifier(String::from_utf8(other.to_vec()).unwrap())
                }
            }
        })
    }
    tokens
}

fn punctuation_u8(v: u8) -> bool {
    v == b'(' || v == b')' || v == b'{' || v == b'}' || v == b';' || v == b',' || v == b'.'
}

fn assemblable_u8(v: u8) -> bool {
    v == b'=' || v == b'-' || v == b'>' || v == b'<' || v == b':' || v == b'+' || v == b'*' || v == b'/'
}

fn itemize(code: &[u8]) -> Vec<&[u8]> {
    let mut items: Vec<&[u8]> = Vec::new();
    let mut i = 0;
    let mut st = 0;
    let mut empty = true;
    while i < code.len() {
        // println!("{i}th round: {}", code[i] as char);
        match code[i] {
            b' ' => {
                if !empty {
                    items.push(&code[st..i]);
                    empty = true;
                }
            }
            v => {
                if punctuation_u8(v) {
                    if !empty {
                        items.push(&code[st..i]);
                        empty = true;
                    }
                    items.push(&code[i..i + 1])
                } else if assemblable_u8(v) {
                    if !empty {
                        if st + 1 == i && !(code[st] as char).is_alphanumeric() {
                            items.push(&code[st..st + 2]);
                            empty = true;
                        } else {
                            items.push(&code[st..i]);
                        }
                    } else {
                        i += 1;
                        let vv = code[i];
                        if vv != b' ' && vv != b'\r' && vv != b'\n' {
                            if assemblable_u8(vv) {
                                items.push(&code[i - 1..i + 1]);
                            } else {
                                items.push(&code[i - 1..i]);
                                empty = false;
                            }
                        } else {
                            items.push(&code[i - 1..i]);
                        }
                    }
                } else if v != b'\r' && v != b'\n' {
                    if empty {
                        st = i;
                        // println!("st changed: {st}");
                    }
                    empty = false;
                }
            }
        }
        i += 1;
    }
    items
}
