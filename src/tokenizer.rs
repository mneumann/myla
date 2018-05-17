#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    OpenPar,
    ClosePar,
    OpenCur,
    CloseCur,
    Semi,
    Keyword(Keyword),
    Ident(String),
    NumLiteral(String),
}

impl Token {
    pub fn get_ident_string(self) -> Option<String> {
        match self {
            Token::Ident(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Keyword {
    Return,
}

const KEYWORDS: &'static [(&'static str, Keyword)] = &[("return", Keyword::Return)];

fn lookup_keyword(ident: &str) -> Option<Keyword> {
    for &(kw_str, kw) in KEYWORDS {
        if ident == kw_str {
            return Some(kw);
        }
    }
    return None;
}

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let char_ary: Vec<char> = s.chars().collect();
    let chars = &char_ary;
    let mut pos = 0;

    loop {
        if let Some(&ch) = chars.get(pos) {
            if ch.is_whitespace() {
                pos += 1;
            } else if ch.is_alphabetic() {
                pos += 1;
                let mut tok = String::new();
                tok.push(ch);
                loop {
                    if let Some(&ch) = chars.get(pos) {
                        if ch.is_alphanumeric() {
                            pos += 1;
                            tok.push(ch);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                tokens.push(
                    lookup_keyword(&tok)
                        .map(|kw| Token::Keyword(kw))
                        .unwrap_or(Token::Ident(tok)),
                );
            } else if ch.is_numeric() {
                pos += 1;
                let mut tok = String::new();
                tok.push(ch);
                loop {
                    if let Some(&ch) = chars.get(pos) {
                        if ch.is_numeric() {
                            pos += 1;
                            tok.push(ch);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                tokens.push(Token::NumLiteral(tok));
            } else {
                match ch {
                    '(' => {
                        pos += 1;
                        tokens.push(Token::OpenPar);
                    }
                    ')' => {
                        pos += 1;
                        tokens.push(Token::ClosePar);
                    }
                    '{' => {
                        pos += 1;
                        tokens.push(Token::OpenCur);
                    }
                    '}' => {
                        pos += 1;
                        tokens.push(Token::CloseCur);
                    }
                    ';' => {
                        pos += 1;
                        tokens.push(Token::Semi);
                    }
                    _ => {
                        panic!("invalid character: {} at position: {}", ch, pos);
                    }
                }
            }
        } else {
            break;
        }
    }

    return tokens;
}

#[test]
fn test_tokenizer() {
    let sourcecode: &str = "
		int main() {
			return 0;
		}
	";

    let exp_tokens = vec![
        Token::Ident("int".to_string()),
        Token::Ident("main".to_string()),
        Token::OpenPar,
        Token::ClosePar,
        Token::OpenCur,
        Token::Keyword(Keyword::Return),
        Token::NumLiteral("0".to_string()),
        Token::Semi,
        Token::CloseCur,
    ];

    let tokens = tokenize(sourcecode);

    assert_eq!(&exp_tokens[..], &tokens[..]);
}
