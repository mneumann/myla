use stream::Stream;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    OpenPar,
    ClosePar,
    OpenCur,
    CloseCur,
    Semi,
    // XXX: Unify Keyword and Ident into Symbol
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
    let mut chars: Stream<char> = Stream::new(s.chars().collect());

    loop {
        if let Some(ch) = chars.peek() {
            if ch.is_whitespace() {
                chars.skip();
            } else if ch.is_alphabetic() {
                let mut tok = String::new();
                loop {
                    match chars.peek() {
                        Some(ch) if ch.is_alphanumeric() => {
                            chars.skip();
                            tok.push(ch);
                        }
                        _ => {
                            break;
                        }
                    }
                }
                assert!(tok.len() > 0);

                tokens.push(
                    lookup_keyword(&tok)
                        .map(|kw| Token::Keyword(kw))
                        .unwrap_or(Token::Ident(tok)),
                );
            } else if ch.is_numeric() {
                // chars.consume_while(|ch| { ch.is_numeric() })
                let mut tok = String::new();
                loop {
                    match chars.peek() {
                        Some(ch) if ch.is_numeric() => {
                            chars.skip();
                            tok.push(ch);
                        }
                        _ => {
                            break;
                        }
                    }
                }
                assert!(tok.len() > 0);
                tokens.push(Token::NumLiteral(tok));
            } else {
                match ch {
                    '(' => {
                        chars.skip();
                        tokens.push(Token::OpenPar);
                    }
                    ')' => {
                        chars.skip();
                        tokens.push(Token::ClosePar);
                    }
                    '{' => {
                        chars.skip();
                        tokens.push(Token::OpenCur);
                    }
                    '}' => {
                        chars.skip();
                        tokens.push(Token::CloseCur);
                    }
                    ';' => {
                        chars.skip();
                        tokens.push(Token::Semi);
                    }
                    _ => {
                        panic!("invalid character: {} at position: {}", ch, chars.get_pos());
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
